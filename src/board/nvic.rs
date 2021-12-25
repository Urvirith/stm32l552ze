// NVIC Description - is on pg 529
// NVIC Registers (Programming Manual) - is on 178

/* Nested vectored interrupt controller (NVIC) */

#[repr(C)]
pub struct NVICReg {
	pub iser:		[u32; 16],		/* Interrupt Set-Enable register */
	pub reserved0:	[u32; 16],
	pub icer:		[u32; 16], 		/* Interrupt Clear-Enable register */
	pub reserved1:	[u32; 16],
	pub ispr:		[u32; 16],    	/* Interrupt Set-Pending Registers */
	pub reserved2:	[u32; 16],
	pub icpr:		[u32; 16],	    /* Interrupt Clear-Pending Registers */
	pub reserved3:	[u32; 16],
	pub iabr:		[u32; 16], 		/* Interrupt Active Bit Registers */
    pub reserved4:	[u32; 16],
	pub itns:		[u32; 16],	    /* Interrupt Target Non-Secure Registers */
	pub reserved5:	[u32; 16],
	pub ipr:		[u8; 480],	    /* Interrupt Priority Registers */
}