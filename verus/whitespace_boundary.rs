use vstd::prelude::*;

verus! {

pub open spec fn is_whitespace(byte: u8) -> bool {
    byte == 32u8 || byte == 9u8
}

pub open spec fn whitespace_end_spec(bytes: Seq<u8>, end: int) -> bool {
    &&& 0 <= end <= bytes.len()
    &&& (end == bytes.len() || !is_whitespace(bytes[end]))
    &&& forall|i: int| 0 <= i < end ==> is_whitespace(bytes[i])
}

pub fn whitespace_end(bytes: &Vec<u8>) -> (end: usize)
    ensures
        whitespace_end_spec(bytes@, end as int),
{
    let mut end = 0;
    while end < bytes.len() && (bytes[end] == 32u8 || bytes[end] == 9u8)
        invariant
            end <= bytes.len(),
            forall|i: int| 0 <= i < end ==> is_whitespace(bytes@[i]),
        decreases bytes.len() - end,
    {
        end += 1;
    }
    end
}

} // verus!

fn main() {}
