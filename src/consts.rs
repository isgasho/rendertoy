use snoozy::*;

snoozy! {
    fn const_f32(_ctx: &mut Context, value: &f32) -> Result<f32> {
        Ok(*value)
    }
}

snoozy! {
    fn const_u32(_ctx: &mut Context, value: &u32) -> Result<u32> {
        Ok(*value)
    }
}
