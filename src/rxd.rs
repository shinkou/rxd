use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

fn dump_file(filename: &str, off: i64, lmt: u64, bpl: u8) -> Result<()>
{
	let mut f = File::open(filename)?;
	let fsize = f.seek(SeekFrom::End(0))?;
	let mut wk: u32 = fsize.try_into().unwrap();
	let mut width = 0;
	while 0 != wk {width += 2; wk >>= 8 & 0xff;}
	let mut idx = off;
	if idx < 0
	{
		idx = off + i64::try_from(fsize).unwrap();
		if idx < 0 {idx = 0;}
	}
	f.seek(SeekFrom::Start(idx.try_into().unwrap()))?;

	let mut buf = [0; 8192];
	let mut n = f.read(&mut buf[..])?;
	let bpl64: u64 = u64::from(bpl);
	let mut cnt: u64 = 0;
	while 0 < n && (lmt > cnt || 0 == lmt)
	{
		for c in &buf[..n]
		{
			if 0 == cnt % bpl64
			{
				print!("{}", format!("{:0width$x}:", idx, width = width));
			}
			print!("{}", format!(" {:02x}", c));
			if bpl64 - 1 == cnt % bpl64 {println!();}
			idx += 1;
			cnt += 1;
			if lmt <= cnt && 0 < lmt {break;}
		}
		n = f.read(&mut buf[..])?;
	}
	if 0 < cnt % bpl64 {println!();}

	Ok(())
}

fn dump_stdin(bpl: u8) -> Result<()>
{
	let bpl64: u64 = u64::from(bpl);
	let mut idx = 0;
	let mut buf = [0; 8192];
	let mut f = std::io::stdin();
	let mut n = f.read(&mut buf[..])?;
	while 0 < n
	{
		for c in &buf[..n]
		{
			if 0 == idx % bpl64 {print!("{}", format!("{:08x}:", idx));}
			print!("{}", format!(" {:02x}", c));
			if bpl64 - 1 == idx % bpl64 {println!();}
			idx += 1;
		}
		n = f.read(&mut buf[..])?;
	}
	if 0 < idx % bpl64 {println!();}

	Ok(())
}

pub fn dump(filename: &str, off: i64, lmt: u64, bpl: u8) -> Result<()>
{
	match filename
	{
		"-" => dump_stdin(bpl)
		, _ => dump_file(filename, off, lmt, bpl)
	}
}
