use std::sync::atomic::{AtomicI32, Ordering};
use std::fmt;
use chrono::{DateTime, Local};

pub static NUM_C_OBJ: AtomicI32 = AtomicI32::new(0);
pub static NUM_D_OBJ: AtomicI32 = AtomicI32::new(0);

pub const SE_UNK: i32 = 0;
pub const SE_SRV: i32 = 1;
pub const SM_SRV: i32 = 2;
pub const SE_PRJ: i32 = 3;
pub const LE_PRJ: i32 = 4;
pub const AL_PRJ: i32 = 5;
pub const SE_MAN: i32 = 6;
pub const SE_MON: i32 = 7;

#[derive(Clone, Debug)]
pub enum Variant {
    Empty,
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
}

// Вывод текста
impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variant::Empty => write!(f, "Null"),
            Variant::Integer(val) => write!(f, "{}", val),
            Variant::Float(val) => write!(f, "{}", val),
            Variant::String(val) => write!(f, "{}", val),
            Variant::Boolean(val) => write!(f, "{}", if *val { "True" } else { "False" }),
        }
    }
}

#[derive(Clone, Debug)]
pub enum MsgPayload {
    Generic {
        val: Variant,
    },
    PropLog {
        prop_name: String,
        unit_name: String,
        new_value: Variant,
        channel_id: u32,
        is_tag: bool,
    },
    ErrLog {
        msg_str: String,
        bug_type: i32,
    },
    ClientPropMod {
        prop_name: String,
        unit_name: String,
        new_value: Variant,
    },
    ConnectLog {
        login: String,
        machine: String,
        connected: bool,
    },
}

#[derive(Clone, Debug)]
pub struct SrvMsg {
    pub msg: i32,
    pub index: i32,
    pub client_id: i32,
    pub msg_date_time: DateTime<Local>,
    pub prj_name: String,
    pub sender_id: Option<usize>,
    pub payload: MsgPayload,
}

impl SrvMsg {
    pub fn new(msg: i32, index: i32, client_id: i32, prj_name: String, payload: MsgPayload) -> Self {
        NUM_C_OBJ.fetch_add(1, Ordering::SeqCst);
        Self {
            msg,
            index,
            client_id,
            msg_date_time: Local::now(),
            prj_name,
            sender_id: None,
            payload,
        }
    }

    pub fn get_client_id(&self) -> Variant {
        Variant::Integer(self.client_id)
    }

    pub fn get_msg_date_time(&self) -> Variant {
        Variant::String(self.msg_date_time.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn get_prj_name(&self) -> Variant {
        Variant::String(self.prj_name.clone())
    }
}

impl Drop for SrvMsg {
    fn drop(&mut self) {
        NUM_D_OBJ.fetch_add(1, Ordering::SeqCst);
    }
}