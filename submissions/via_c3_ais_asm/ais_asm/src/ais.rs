use std::collections::VecDeque;

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
    MissingSubOp(Instruction),

    DecodeError(Vec<u8>),

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
                "R4"  => Ok(4),
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    XJ,

    // I type
    ORIU,
    ADDI,
    ANDIU,
    ANDIL,
    ANDI,
    ORI,
    XORI,
    XORIU,

    // XALU type
    XALU,
    XALUI,
    XALUR,
    XALUIR,

    XMISC,
    XLEAI,
    XLEAD,
    XL,
    XL2,
    XL3,
    XLBI,
    XLDESC,
    XIOR,
    XPOPBR,
    XPOP,
    XS,
    XS2,
    XPUSHI,
    XSI,
    XPUSHIP,
    XIOW,
    XSU,
    XPUSH,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub enum SubOp {
    SHL,
    SHR,
    SAR,
    ROL,
    ROR,
    RCL,
    RCR,
    INC,
    CMPS,
    DEC,
    IMUL,
    MUL,
    IDIV,
    ADD,
    ADC,
    SUB,
    SBB,
    AND,
    OR,
    XOR,
    NOR,
    CTC2,
    SETCC,
    MFLOU,
    MFLOI,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rs: Option<Register>,
    pub rt: Option<Register>,
    pub rd: Option<Register>,
    pub imm: Option<u16>,
    pub constant: Option<Const>,
    pub subop: Option<SubOp>,
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
            subop: None,
        }
    }

    pub fn j(base: Register) -> Self {
        let mut ret = Self::new(Opcode::XJ);
        ret.rt = Some(base);
        ret
    }

    pub fn i_type(opcode: Opcode, dst: Register, src: Register, imm: u16) -> Self {
        let mut ret = Self::new(opcode);
        ret.rs = Some(src);
        ret.rt = Some(dst);
        ret.imm = Some(imm);
        ret
    }

    pub fn xalu_type(subop: SubOp, dst: Register, src: Register, extra: Register) -> Self {
        let mut ret = Self::new(Opcode::XALUR);
        ret.rs = Some(src);
        ret.rd = Some(dst);
        ret.rt = Some(extra);
        ret.subop = Some(subop);
        ret
    }

    pub fn xalui_type(subop: SubOp, dst: Register, src: Register, constant: Const) -> Self {
        let mut ret = Self::new(Opcode::XALUIR);
        ret.rs = Some(src);
        ret.rd = Some(dst);
        ret.constant = Some(constant);
        ret.subop = Some(subop);
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
        let bits: u32 = match self.opcode {
            Opcode::XJ => 0o6,

            Opcode::ORIU => 0o10,
            Opcode::ADDI => 0o11,
            Opcode::ANDIU => 0o12,
            Opcode::ANDIL => 0o13,
            Opcode::ANDI => 0o14,
            Opcode::ORI => 0o15,
            Opcode::XORI => 0o16,
            Opcode::XORIU => 0o17,

            Opcode::XALU => 0o40,
            Opcode::XALUI => 0o41,
            Opcode::XALUR => 0o42,
            Opcode::XALUIR => 0o43,
            _ => return Err(AisError::Unsupported(self.clone())),
        };

        Ok(bits << 26)
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

    fn const_bits(&self) -> Result<u32, AisError> {
        let c = self
            .constant
            .ok_or_else(|| AisError::MissingConstant(self.clone()))?;

        todo!();

        Ok(0)
    }

    fn encode_subop(&self) -> Result<u32, AisError> {
        let subop = self
            .subop
            .ok_or_else(|| AisError::MissingSubOp(self.clone()))?;
        let bits = match subop {
            SubOp::SHL => 0,
            SubOp::SHR => 2,
            SubOp::SAR => 3,
            SubOp::ROL => 4,
            SubOp::ROR => 5,
            SubOp::RCL => 6,
            SubOp::RCR => 7,
            SubOp::INC => 8,
            SubOp::CMPS => 9,
            SubOp::DEC => 10,
            SubOp::IMUL => 12,
            SubOp::MUL => 13,
            SubOp::IDIV => 14,
            SubOp::ADD => 16,
            SubOp::ADC => 17,
            SubOp::SUB => 18,
            SubOp::SBB => 19,
            SubOp::AND => 20,
            SubOp::OR => 21,
            SubOp::XOR => 22,
            SubOp::NOR => 23,
            SubOp::CTC2 => 25,
            SubOp::SETCC => 29,
            SubOp::MFLOU => 30,
            SubOp::MFLOI => 31,
        };

        Ok(bits)
    }

    pub fn encode(&self) -> Result<Vec<u8>, AisError> {
        let instr =
            if self.is_i_type() {
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
                let subop = self.encode_subop()?;

                op | rs | rt | rd | subop
            } else if self.is_xalui_type() {
                let op = self.encode_opcode()?;
                let rs = self.encode_rs()?;
                let c = self.const_bits()?;
                let rd = self.encode_rd()?;
                let subop = self.encode_subop()?;

                op | rs | c | rd | subop
            } else if self.opcode == Opcode::XJ {
                let op = self.encode_opcode()?;
                let rt = self.encode_rt()?;

                op | rt | 0b01_0001_11
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
            instr.subop = Some(decode_subop(word)?);
            instr.rs = Some(Register::Index(rs_bits));
            instr.rt = Some(Register::Index(rt_bits));
            instr.rd = Some(Register::Index(rd_bits));
        } else if instr.is_xalui_type() {
            instr.subop = Some(decode_subop(word)?);
            instr.rs = Some(Register::Index(rs_bits));
            instr.rd = Some(Register::Index(rd_bits));
        } else if instr.opcode == Opcode::XJ {
            instr.rt = Some(Register::Index(rt_bits));
        } else {
            return Err(AisError::DecodeError(bytes.into()));
        }

        Ok((instr, 6))
    }
}

fn decode_subop(word: u32) -> Result<SubOp, AisError> {
    let subop_bits = word & 0x1F;
    let subop = match subop_bits {
        0 => SubOp::SHL,
        2 => SubOp::SHR,
        3 => SubOp::SAR,
        4 => SubOp::ROL,
        5 => SubOp::ROR,
        6 => SubOp::RCL,
        7 => SubOp::RCR,
        8 => SubOp::INC,
        9 => SubOp::CMPS,
        10 => SubOp::DEC,
        12 => SubOp::IMUL,
        13 => SubOp::MUL,
        14 => SubOp::IDIV,
        16 => SubOp::ADD,
        17 => SubOp::ADC,
        18 => SubOp::SUB,
        19 => SubOp::SBB,
        20 => SubOp::AND,
        21 => SubOp::OR,
        22 => SubOp::XOR,
        23 => SubOp::NOR,
        25 => SubOp::CTC2,
        29 => SubOp::SETCC,
        30 => SubOp::MFLOU,
        31 => SubOp::MFLOI,
        _ => return Err(AisError::UnknownSubOp(subop_bits)),
    };

    Ok(subop)
}

fn decode_opcode(word: u32) -> Result<Opcode, AisError> {
    let opcode_bits = (word >> 26) & 0x3F;

    let opcode = match opcode_bits {
        0o06 => Opcode::XJ,

        0o10 => Opcode::ORIU,
        0o11 => Opcode::ADDI,
        0o12 => Opcode::ANDIU,
        0o13 => Opcode::ANDIL,
        0o14 => Opcode::ANDI,
        0o15 => Opcode::ORI,
        0o16 => Opcode::XORI,
        0o17 => Opcode::XORIU,

        0o40 => Opcode::XALU,
        0o41 => Opcode::XALUI,
        0o42 => Opcode::XALUR,
        0o43 => Opcode::XALUIR,

        0o50 => Opcode::XMISC,
        0o53 => Opcode::XLEAI,
        0o54 => Opcode::XLEAD,

        0o60 => Opcode::XL,
        0o61 => Opcode::XL2,
        0o62 => Opcode::XL3,
        0o63 => Opcode::XLBI,
        0o64 => Opcode::XLDESC,
        0o65 => Opcode::XIOR,
        0o66 => Opcode::XPOPBR,
        0o67 => Opcode::XPOP,

        0o70 => Opcode::XS,
        0o71 => Opcode::XS2,
        0o72 => Opcode::XPUSHI,
        0o73 => Opcode::XSI,
        0o74 => Opcode::XPUSHIP,
        0o75 => Opcode::XIOW,
        0o76 => Opcode::XSU,
        0o77 => Opcode::XPUSH,

        _ => return Err(AisError::UnknownOpcode(opcode_bits)),
    };

    Ok(opcode)
}
