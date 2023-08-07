use lamukoi::structures::*;
use lamukoi::*;
use std::collections::HashMap;
use std::io::{Read, Write};

pub(crate) fn prelude() -> Program {
    program![
        #EQ x y;
        #ADD x y;
        #SUB x y;
        True t f = t;
        False t f = f;
        // Scott list
        SNil nil cons = nil;
        SCons x xs nil cons = cons x xs;
        // Church list (right fold)
        cNil = |c n| n;
        cCons = |h t c n| c h (t c n);
        zero = |f x| x;
        succ = |n f x| f (n f x);
        int2cNat i = EQ i 0 zero (succ (int2cNat (SUB i 1)));
        cNat2Int x = x (ADD 1) 0;
        #READ; // reads more from input as necessary, turning into cBit1/cBit0/cNil
        // which eventually expands into Church list of bits
        cBit1 = cCons True READ;
        cBit0 = cCons False READ;
        cList2sList clist = clist SCons SNil;
        // Scott list of bits is CList2SList READ
        #SHOW x; // puts current bit to the stream, returning sShow
        sShow stream = stream id (|item xs| SHOW (item 1 0) xs);
        cShow stream = sShow (cList2sList stream);
        id x = x;
    ]
}

struct InputDevice<I: Read> {
    bytes: std::io::Bytes<I>,
    cur_byte: u8,
    cur_bit: u8,
}

impl<I: Read> InputDevice<I> {
    fn new(device: I) -> Self {
        Self {
            bytes: device.bytes(),
            cur_byte: 0,
            cur_bit: 0,
        }
    }

    fn read(&mut self) -> Option<bool> {
        if self.cur_bit == 0 {
            self.cur_byte = self.bytes.next().map(|res| res.unwrap())?;
            let bit = self.cur_byte % 2;
            self.cur_bit = 1;
            Some(bit == 1)
        } else {
            let bit = (self.cur_byte >> self.cur_bit) & 1;
            self.cur_bit += 1;
            if self.cur_bit == 8 { self.cur_bit = 0; }
            Some(bit == 1)
        }
    }
}

struct OutputDevice<O: Write> {
    device: O,
    cur_byte: u8,
    cur_bit: u8,
}

impl<O: Write> OutputDevice<O> {
    fn new(device: O) -> Self {
        Self {
            device,
            cur_byte: 0,
            cur_bit: 0,
        }
    }

    fn write(&mut self, bit: u8) {
        self.cur_byte |= bit << self.cur_bit;
        self.cur_bit += 1;
        if self.cur_bit == 8 {
            self.cur_bit = 0;
            self.device.write(&[self.cur_byte]).unwrap();
            self.cur_byte = 0;
        }
    }
}

pub(crate) fn prelude_defs<'a, 'b, 'c, I, O>(input: &'a mut I, output: &'b mut O, table: &HashMap<&str, usize>) -> HashMap<&'static str, Primop<'c>>
where I: Read + 'static, O: Write + 'static, 'a: 'c, 'b: 'c {
    let eq = |arr: &[i64]| Some(Atom::Prim((arr[0] == arr[1]) as i64));
    let add = |arr: &[i64]| Some(Atom::Prim(arr[0] + arr[1]));
    let sub = |arr: &[i64]| Some(Atom::Prim(arr[0] - arr[1]));
    let mut id1 = InputDevice::new(input);
    let cbit1 = table["cBit1"];
    let cbit0 = table["cBit0"];
    let cnil = table["cNil"];
    let cread = move |_arr: &[i64]|
        if let Some(bit) = id1.read() {
            if bit { Some(Atom::Sc(cbit1)) } else { Some(Atom::Sc(cbit0)) }
        } else { Some(Atom::Sc(cnil)) };
    let mut od = OutputDevice::new(output);
    let show = table["sShow"];
    let show = move |arr: &[i64]| {
        od.write(arr[0] as u8);
        Some(Atom::Sc(show))
    };
    HashMap::from([
        ("EQ", Box::new(eq) as _),
        ("ADD", Box::new(add) as _),
        ("SUB", Box::new(sub) as _),
        ("READ", Box::new(cread) as _),
        ("SHOW", Box::new(show) as _),
    ])
}