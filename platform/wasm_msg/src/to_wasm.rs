use makepad_live_id::*;
use crate::from_wasm::*;

pub trait ToWasm {
    fn u32_size() -> usize;
    
    fn type_name()->&'static str{panic!()}
    fn live_id()->LiveId{panic!()}

    fn to_wasm(inp: &mut ToWasmMsg) -> Self;
    fn to_wasm_js_body(out: &mut String, prop: &str, nest:usize);

    fn to_wasm_js_method(out: &mut String) {
        let id = Self::live_id();
        out.push_str(&format!("
            {}(obj){{
            let app = this.app;
            this.reserve_u32({});
            app.u32[this.u32_offset ++] = {};
            app.u32[this.u32_offset ++] = {};
            let block_len_offset = this.u32_offset ++;
            \n", Self::type_name(), 3 + Self::u32_size(), id.0 & 0xffff_ffff, (id.0 >> 32)));
        
        Self::to_wasm_js_body(out, "obj", 0);
        
        out.push_str("
            if( (this.u32_offset & 1) != 0){ app.u32[this.u32_offset ++] = 0;}
            let new_len = (this.u32_offset - this.u32_ptr) >> 1;
            app.u32[block_len_offset] = new_len - app.u32[this.u32_ptr + 1];
            app.u32[this.u32_ptr + 1] = new_len;
            }
        ");
    }
}

pub struct ToWasmMsg {
    data: Vec<u64>,
    pub u32_offset: usize
}

pub struct ToWasmCmdSkip{
    len:usize,
    base:usize
}

impl ToWasmMsg {
    
    pub fn from_wasm_ptr(val: u32) -> Self {
        unsafe {
            let ptr = val as *mut u64;
            let head = ptr.offset(0).read();
            let len = (head >> 32) as usize;
            let cap = (head & 0xffff_ffff) as usize;
            Self {
                data: Vec::from_raw_parts(ptr, len, cap),
                u32_offset: 2,
            }
        }
    }
    
    pub fn into_from_wasm(self) -> FromWasmMsg {
        FromWasmMsg {
            data: self.data,
            odd: false
        }
    }
    
    pub fn read_u32(&mut self) -> u32 {
        let ret = if self.u32_offset & 1 != 0 {
            (self.data[self.u32_offset >> 1] >> 32) as u32
        }
        else {
            (self.data[self.u32_offset >> 1] & 0xffff_ffff) as u32
        };
        self.u32_offset += 1;
        ret
    }
    
    pub fn read_cmd_skip(&mut self)->ToWasmCmdSkip{
        ToWasmCmdSkip{
            base: self.u32_offset >> 1,
            len: self.read_u32() as usize, 
        }
    }
    
    pub fn cmd_skip(&mut self, cmd_skip:ToWasmCmdSkip){
        self.u32_offset = (cmd_skip.base + cmd_skip.len - 1)<<1
    }
    
    pub fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }
    
    pub fn read_u64(&mut self) -> u64 {
        self.u32_offset += self.u32_offset & 1;
        let ret = self.data[self.u32_offset >> 1];
        self.u32_offset += 2;
        ret
    }
    
    pub fn read_f64(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }
    
    pub fn read_string(&mut self) -> String {
        let chars = self.read_u32();
        let mut out = String::new();
        for _ in 0..chars {
            out.push(char::from_u32(self.read_u32()).unwrap_or('?'));
        }
        out
    }
    
    pub fn was_last_cmd(&mut self)->bool{
        self.u32_offset += self.u32_offset & 1;
        self.u32_offset>>1 >= self.data.len()
    }
}
