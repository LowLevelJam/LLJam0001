use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug)]
pub enum AisError {
    InvalidRegisterIndex(u8),
    InvalidRegisterName(String),

    Unsupported(Instruction),

    MissingImmediate(Instruction),
    MissingRs(Instruction),
    MissingRt(Instruction),
    MissingRd(Instruction),
    MissingConstant(Instruction),
    MissingOffset(Instruction),
    MissingFunction(Instruction),

    DecodeError(Vec<u8>),
    DecodeIssue,

    UnknownOpcode(u32),
    UnknownSubOp(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Index(u8),
    Name(String),
}

impl Register {
    fn bits(&self) -> Result<u32, AisError> {
        match self {
            Register::Index(x) if *x > 31 => Err(AisError::InvalidRegisterIndex(*x)),
            Register::Index(x) => Ok((*x).into()),
            Register::Name(x) => match x.as_str() {
                "R4" => Ok(4),
                "EAX" => Ok(16),
                "ECX" => Ok(17),
                "EDX" => Ok(18),
                "EBX" => Ok(19),
                _ => Err(AisError::InvalidRegisterName(x.clone())),
            },
        }
    }
}

impl From<u8> for Register {
    fn from(x: u8) -> Self {
        Register::Index(x)
    }
}

impl From<&str> for Register {
    fn from(x: &str) -> Self {
        Register::Name(x.to_string())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Const {
    Number(i8),
    // There are some other special case, skip for now
}

#[derive(Debug, Copy, Clone)]
pub enum Offset {
    Number(i8),
    // There are some other special case, skip for now
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum Size {
    Bits16 = 0b000,
    Bits32 = 0b010,
    Bits8 = 0b001,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum Sel {
    Flat = 0b1010,
}

#[derive(Debug, Copy, Clone)]
pub enum SubOpXls {
    Xio(SubOpXio),
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum SubOpXio {
    Norm = 0,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum AddrSize {
    Bits32 = 0b11,
}

#[derive(Debug, Copy, Clone)]
pub enum Function {
    Xls(SubOpXls, AddrSize, Size, Sel),
    Xalu(SubOpXalu, DpCntl),
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum DpCntl {
    Word = 0b000,
    Short = 0b001,
    LL = 0b010,
    HL = 0b011,
    LH = 0b100,
    HH = 0b101,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum Opcode {
    XJ = 0o06,

    // I type
    ORIU = 0o10,
    ADDI = 0o11,
    ANDIU = 0o12,
    ANDIL = 0o13,
    ANDI = 0o14,
    ORI = 0o15,
    XORI = 0o16,
    XORIU = 0o17,

    // XALU type
    XALU = 0o40,
    XALUI = 0o41,
    XALUR = 0o42,
    XALUIR = 0o43,

    XMISC = 0o50,
    XLEAI = 0o53,
    XLEAD = 0o54,

    XL = 0o60,
    XL2 = 0o61,
    XL3 = 0o62,
    XLBI = 0o63,
    XLDESC = 0o64,
    XIOR = 0o65,
    XPOPBR = 0o66,
    XPOP = 0o67,

    XS = 0o70,
    XS2 = 0o71,
    XPUSHI = 0o72,
    XSI = 0o73,
    XPUSHIP = 0o74,
    XIOW = 0o75,
    XSU = 0o76,
    XPUSH = 0o77,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum SubOpXalu {
    SHL = 0o00,
    SHR = 0o02,
    SAR = 0o03,
    ROL = 0o04,
    ROR = 0o05,
    RCL = 0o06,
    RCR = 0o07,
    INC = 0o10,
    CMPS = 0o11,
    DEC = 0o12,
    IMUL = 0o14,
    MUL = 0o15,
    IDIV = 0o16,
    ADD = 0o20,
    ADC = 0o21,
    SUB = 0o22,
    SBB = 0o23,
    AND = 0o24,
    OR = 0o25,
    XOR = 0o26,
    NOR = 0o27,
    CTC2 = 0o31,
    SETCC = 0o35,
    MFLOU = 0o36,
    MFLOI = 0o37,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rs: Option<Register>,
    pub rt: Option<Register>,
    pub rd: Option<Register>,
    pub imm: Option<u16>,
    pub constant: Option<Const>,
    pub offset: Option<Offset>,
    pub function: Option<Function>,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self {
            opcode,
            rs: None,
            rt: None, // base
            rd: None,
            imm: None,
            constant: None,
            offset: None,
            function: None,
        }
    }



    pub fn i_type(opcode: Opcode, dst: Register, src: Register, imm: u16) -> Self {
        let mut ret = Self::new(opcode);
        ret.rs = Some(src);
        ret.rt = Some(dst);
        ret.imm = Some(imm);
        ret
    }

    pub fn xalur(
        subop: SubOpXalu,
        dpcntl: DpCntl,
        dst: Register,
        src: Register,
        extra: Register,
    ) -> Self {
        let mut ret = Self::new(Opcode::XALUR);
        ret.rs = Some(src);
        ret.rd = Some(dst);
        ret.rt = Some(extra);
        ret.function = Some(Function::Xalu(subop, dpcntl));
        ret
    }

    pub fn xaluir(
        subop: SubOpXalu,
        dpcntl: DpCntl,
        dst: Register,
        src: Register,
        constant: Const,
    ) -> Self {
        let mut ret = Self::new(Opcode::XALUIR);
        ret.rs = Some(src);
        ret.rd = Some(dst);
        ret.constant = Some(constant);
        ret.function = Some(Function::Xalu(subop, dpcntl));
        ret
    }

    pub fn xiow(size: Size, port: Register, value: Register) -> Self {
        let mut instr = Instruction::xls_type(Opcode::XIOW, value, port, Offset::Number(0));
        instr.function = Some(Function::Xls(
            SubOpXls::Xio(SubOpXio::Norm),
            AddrSize::Bits32,
            size,
            Sel::Flat,
        ));
        instr
    }

    pub fn xior(size: Size, port: Register, value: Register) -> Self {
        let mut instr = Instruction::xls_type(Opcode::XIOR, value, port, Offset::Number(0));
        instr.function = Some(Function::Xls(
            SubOpXls::Xio(SubOpXio::Norm),
            AddrSize::Bits32,
            size,
            Sel::Flat,
        ));
        instr
    }

    pub fn xj(base: Register) -> Self {
        let mut ret = Self::new(Opcode::XJ);
        ret.rt = Some(base);
        ret
    }

    pub fn xls_type(opcode: Opcode, rs: Register, base: Register, offset: Offset) -> Self {
        let mut ret = Self::new(opcode);
        ret.rs = Some(rs);
        ret.rt = Some(base);
        ret.offset = Some(offset);
        ret
    }

    fn is_i_type(&self) -> bool {
        matches!(
            self.opcode,
            Opcode::ORIU
                | Opcode::ADDI
                | Opcode::ANDIU
                | Opcode::ANDIL
                | Opcode::ANDI
                | Opcode::ORI
                | Opcode::XORI
                | Opcode::XORIU
        )
    }

    fn is_xalu_type(&self) -> bool {
        matches!(self.opcode, Opcode::XALU | Opcode::XALUR)
    }

    fn is_xalui_type(&self) -> bool {
        matches!(self.opcode, Opcode::XALUI | Opcode::XALUIR)
    }

    fn encode_opcode(&self) -> Result<u32, AisError> {
        Ok((self.opcode as u32) << 26)
    }

    fn encode_rs(&self) -> Result<u32, AisError> {
        self.rs
            .as_ref()
            .ok_or_else(|| AisError::MissingRs(self.clone()))
            .and_then(|x| x.bits())
            .map(|x| x << 21)
    }

    fn encode_rt(&self) -> Result<u32, AisError> {
        self.rt
            .as_ref()
            .ok_or_else(|| AisError::MissingRt(self.clone()))
            .and_then(|x| x.bits())
            .map(|x| x << 16)
    }

    fn encode_rd(&self) -> Result<u32, AisError> {
        self.rd
            .as_ref()
            .ok_or_else(|| AisError::MissingRd(self.clone()))
            .and_then(|x| x.bits())
            .map(|x| x << 11)
    }

    fn encode_imm(&self) -> Result<u32, AisError> {
        self.imm
            .ok_or_else(|| AisError::MissingImmediate(self.clone()))
            .map(|x| x.into())
    }

    fn encode_const(&self) -> Result<u32, AisError> {
        let c = self
            .constant
            .ok_or_else(|| AisError::MissingConstant(self.clone()))?;

        Ok(0 /*c as u32*/)
    }

    fn encode_offset(&self) -> Result<u32, AisError> {
        let offset = self
            .offset
            .ok_or_else(|| AisError::MissingOffset(self.clone()))?;

        let offset_bits = match offset {
            Offset::Number(0) => 0,
            _ => return Err(AisError::Unsupported(self.clone())),
        };

        Ok(offset_bits)
    }

    fn encode_sub_op_xls(&self, subop: SubOpXls) -> Result<u32, AisError> {
        let bits = match subop {
            SubOpXls::Xio(x) => x as u32,
        };

        Ok(bits << 9)
    }

    fn encode_function(&self) -> Result<u32, AisError> {
        let function = self
            .function
            .ok_or_else(|| AisError::MissingFunction(self.clone()))?;

        let bits = match function {
            Function::Xalu(sub_op, dp_cntl) => (sub_op as u32) | (dp_cntl as u32) << 5,
            Function::Xls(sub_op, addr_size, size, sel) => {
                let subop_bits = self.encode_sub_op_xls(sub_op)?;
                subop_bits
                    | (addr_size as u32 & 2) << 7
                    | ((size as u32) & 0x6) << 5
                    | (sel as u32) << 2
                    | (size as u32 & 1) << 1
                    | addr_size as u32 & 1
            }
        };

        Ok(bits)
    }

    pub fn encode(&self) -> Result<Vec<u8>, AisError> {
        let instr = if self.is_i_type() {
            let op = self.encode_opcode()?;
            let rs = self.encode_rs()?;
            let rt = self.encode_rt()?;
            let imm = self.encode_imm()?;

            op | rs | rt | imm
        } else if self.is_xalu_type() {
            let op = self.encode_opcode()?;
            let rs = self.encode_rs()?;
            let rt = self.encode_rt()?;
            let rd = self.encode_rd()?;
            let function = self.encode_function()?;

            op | rs | rt | rd | function
        } else if self.is_xalui_type() {
            let op = self.encode_opcode()?;
            let rs = self.encode_rs()?;
            let c = self.encode_const()?;
            let rd = self.encode_rd()?;
            let function = self.encode_function()?;

            op | rs | c | rd | function
        } else if self.opcode == Opcode::XJ {
            let op = self.encode_opcode()?;
            let rt = self.encode_rt()?;

            op | rt | 0b01_0001_00
        } else if matches!(self.opcode, Opcode::XIOR | Opcode::XIOW) {
            let op = self.encode_opcode()?;
            let rs = self.encode_rs()?;
            let base = self.encode_rt()?;
            let offset = self.encode_offset()?;
            let function = self.encode_function()?;

            op | rs | base | offset | function
        } else {
            return Err(AisError::Unsupported(self.clone()));
        };

        let mut data = Vec::new();
        data.extend_from_slice(&[0x62, 0x80]);
        data.extend_from_slice(&instr.to_le_bytes());
        Ok(data)
    }

    pub fn decode(bytes: &[u8]) -> Result<(Instruction, usize), AisError> {
        if bytes.len() < 6 {
            return Err(AisError::DecodeError(bytes.into()));
        }

        let header = &bytes[0..2];
        if header != [0x62, 0x80] {
            return Err(AisError::DecodeError(bytes.into()));
        }

        let word = u32::from_le_bytes(bytes[2..6].try_into().unwrap());

        let opcode = decode_opcode(word)?;
        let mut instr = Instruction::new(opcode);

        let rs_bits = ((word >> 21) & 0x1F).try_into().unwrap();
        let rt_bits = ((word >> 16) & 0x1F).try_into().unwrap();
        let rd_bits = ((word >> 11) & 0x1F).try_into().unwrap();
        let imm_bits = (word & 0xFFFF).try_into().unwrap();

        if instr.is_i_type() {
            instr.rs = Some(Register::Index(rs_bits));
            instr.rt = Some(Register::Index(rt_bits));
            instr.imm = Some(imm_bits);
        } else if instr.is_xalu_type() {
            instr.function = Some(decode_xalu_function(word)?);
            instr.rs = Some(Register::Index(rs_bits));
            instr.rt = Some(Register::Index(rt_bits));
            instr.rd = Some(Register::Index(rd_bits));
        } else if instr.is_xalui_type() {
            instr.function = Some(decode_xalu_function(word)?);
            instr.rs = Some(Register::Index(rs_bits));
            instr.rd = Some(Register::Index(rd_bits));
        } else if instr.opcode == Opcode::XJ {
            instr.rt = Some(Register::Index(rt_bits));
        } else if matches!(instr.opcode, Opcode::XIOR | Opcode::XIOW) {
            instr.rs = Some(Register::Index(rs_bits));
            instr.rt = Some(Register::Index(rt_bits));
            instr.offset = Some(Offset::Number(0)); //FIXME
            //instr.function = Some()

        } else {
            return Err(AisError::DecodeError(bytes.into()));
        }

        Ok((instr, 6))
    }
}

fn decode_xalu_function(word: u32) -> Result<Function, AisError> {
    let sub_op_bits = word & 0x1F;
    let dp_cntl_bits = (word >> 5) & 0x3;
    let sub_op = FromPrimitive::from_u32(sub_op_bits).ok_or(AisError::DecodeIssue)?;
    let dp_cntl = FromPrimitive::from_u32(dp_cntl_bits).ok_or(AisError::DecodeIssue)?;
    Ok(Function::Xalu(sub_op, dp_cntl))
}

fn decode_opcode(word: u32) -> Result<Opcode, AisError> {
    let opcode_bits = (word >> 26) & 0x3F;
    FromPrimitive::from_u32(opcode_bits).ok_or(AisError::UnknownOpcode(opcode_bits))
}
