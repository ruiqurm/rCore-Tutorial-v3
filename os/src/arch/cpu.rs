use core::arch::asm;
pub fn cpu_id() -> usize {
	let id;
	unsafe{
		asm!("mv {0}, x4",
			out(reg) id
		);
	}
	id
}