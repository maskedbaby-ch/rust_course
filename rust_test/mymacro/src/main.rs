use std::ops::Add;
macro_rules! myvec {
	($($x:expr),*) => {
		{
			let mut tvec = Vec::new();
			$(tvec.push($x);)*
			tvec
		}
	}
}

pub struct SaturatingU16 {
    val: u16
}

macro_rules! impl_add_for_saturating_u16 {
	($l:ty, $r:ty) => {
		impl Add<$r> for $l {
			type Output = SaturatingU16;
			fn add(self, other: $r) -> Self::Output {
				SaturatingU16 { val: self.val.saturating_add(other) }
			}
		}
	};
	($l:ty, $r:ty, $x:ident, $y:ident) => {
		impl Add<$r> for $l {
			type Output = SaturatingU16;
			fn add(self, other: $r) -> Self::Output {
				SaturatingU16 { val: self.$x.saturating_add(other.$y) }
			}
		}
	};
}

impl_add_for_saturating_u16!(SaturatingU16, SaturatingU16, val, val);
impl_add_for_saturating_u16!(SaturatingU16, u16);
impl_add_for_saturating_u16!(&SaturatingU16, SaturatingU16, val, val);
impl_add_for_saturating_u16!(SaturatingU16, &SaturatingU16, val, val);



fn main() {
	let a = myvec![1,2,3];
    println!("Hello, world! {:?}", a);
}
