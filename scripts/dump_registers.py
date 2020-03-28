# -*- coding: utf-8 -*-

"""
A helper script to dump MMIO registers from the Tegra X1 TRM.

Useful for quickly emiting Rust abstractions through register-rs.

Arguments
---------
manual : str
    Path to the TRM PDF file.
output : str
    Path to the Rust output source file. Defaults to ``registers.rs``.
-p : List[int]
    List of page numbers from where registers should be dumped.

Dependencies
------------
pdfminer
    Parsing and extracting PDF contents.
"""

import argparse
from collections import namedtuple
from enum import Enum
import re

from pdfminer.converter import PDFPageAggregator
from pdfminer.layout import LAParams, LTTextBoxHorizontal
from pdfminer.pdfinterp import PDFPageInterpreter, PDFResourceManager
from pdfminer.pdfpage import PDFPage

# TODO: Add support for padding out gaps between registers.

FMT = """use register::{{mmio::*, register_structs}};

register_structs! {{
    #[allow(non_snake_case)]
    pub Registers {{
        {}
    }}
}}

"""

# A Regular Expression for extracting registers from a headline.
# First match group: The chapter.
# Second match group: Register name.
# Third match group: Register short description (if present, otherwise garbage or nothing).
REGISTER_PATTERN = re.compile(r'(\d[\d\.]+\d)\s+(\w+)(?:\n(.+))?\n', re.MULTILINE)

# A Regular Expression for extracting metadata corresponding to a register.
# First match: The register offset.
# Second match: The permissions of the memory segment.
REGISTER_META_PATTERN = re.compile(r'Offset:\s+([0-9a-fA-Fx]+)\s+[\|I]\s+Read\/Write:\s([\w\/]+)')


class MemoryPermissions(Enum):
    RO = 'ReadOnly<u32>'

    RW = 'ReadWrite<u32>'

    WO = 'WriteOnly<u32>'

    @classmethod
    def from_str(cls, permissions):
        permissions = permissions.upper()  # Better safe than sorry.
        if permissions.startswith(('R/W', 'RW', 'Read/Write', 'RWC')):
            return cls.RW
        elif permissions.startswith('RO'):
            return cls.RO
        elif permissions.startswith('WO'):
            return cls.WO
        else:
            raise ValueError()

    def emit_code(self):
        return self.value


class Register(namedtuple('Register', 'name offset perms')):
    __slots__ = ()

    def __new__(cls, name, offset, perms):
        offset = f'0x{int(offset, 16):X}'  # Style convention for hex numbers.
        perms = MemoryPermissions.from_str(perms)

        return super().__new__(cls, name, offset, perms)

    @classmethod
    def from_regex_match(cls, register, register_meta):
        return cls(
            name=register.group(2),
            offset=register_meta.group(1),
            perms=register_meta.group(2)
        )

    def emit_code(self):
        return f'({self.offset} => pub {self.name}: {self.perms.emit_code()}),'


def extract_registers(layout):
    registers = []
    register_metas = []

    for element in layout:
        # We only care about text boxes.
        if isinstance(element, LTTextBoxHorizontal):
            element = element.get_text()
        else:
            continue

        # Extract all registers and meta information from a page/layout.

        register = REGISTER_PATTERN.search(element)
        if register:
            # This is a dirty hack due to Nvidia's stupid layout rules. Nvidia
            # rarely uses sequences of numbers separated by \n characters
            # which are matched by this regex as well. Since these matches
            # are no actual registers, they need to be filtered out.
            if not re.fullmatch(r'(?:\d+\n)*', register.group(0)):
                registers.append(register)

        meta = REGISTER_META_PATTERN.search(element)
        if meta:
            register_metas.append(meta)

    return registers, register_metas


def generate_code(output, register_matches, register_meta_matches):
    registers = []

    # Every register should have corresponding meta information and vice versa.
    assert len(register_matches) == len(register_meta_matches)

    # Parse and store all the registers.
    while True:
        try:
            register = register_matches.pop(0)
            register_meta = register_meta_matches.pop(0)

            registers.append(Register.from_regex_match(register, register_meta))
        except IndexError:  # Lists are empty, we're done.
            break

    # Generate the register_structs! terminator.
    final_offset = int(registers[-1].offset, 16) + 4
    terminator = f'(0x{final_offset:X} => @END),'

    # Generate Rust code.
    register_structs_members = [
        reg.emit_code()
        for reg in sorted(registers, key=lambda reg: int(reg.offset, 16))
    ]
    register_structs_members.append(terminator)
    code = FMT.format('\n'.join(register_structs_members))

    # Write result to output file.
    # TODO: Run rustfmt on the file?
    with open(output, 'w') as f:
        f.write(code)


def main(parser, args):
    resource_manager = PDFResourceManager()
    layout_params = LAParams()
    device = PDFPageAggregator(resource_manager, laparams=layout_params)
    interpreter = PDFPageInterpreter(resource_manager, device)

    trm = open(args.manual, 'rb')

    # Lists to keep track of registers and corresponding meta information.
    register_matches = []
    register_meta_matches = []

    for page in PDFPage.get_pages(trm, pagenos=args.p):
        # Compute the layout of the page.
        interpreter.process_page(page)
        layout = device.get_result()

        # Extract and cache the registers of the page by regex.
        registers, register_metas = extract_registers(layout)
        register_matches.extend(registers)
        register_meta_matches.extend(register_metas)

    trm.close()

    # Finally emit Rust code and store it in the output file.
    generate_code(args.output, register_matches, register_meta_matches)


def parse_args():
    parser = argparse.ArgumentParser(
        description='Helper tool for dumping registers from the TRM and emiting Rust abstractions.'
    )
    parser.set_defaults(func=main)

    parser.add_argument('manual', help='Path to TRM PDF.')
    parser.add_argument('output', help='Path to output file.', nargs='?', default='registers.rs')
    parser.add_argument('-p', help='List of pages to dump.', type=int, nargs='+', required=True)

    return parser, parser.parse_args()


if __name__ == '__main__':
    parser, args = parse_args()
    args.func(parser, args)
