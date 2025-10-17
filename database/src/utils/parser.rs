pub fn skip_whitespace(v: &[u8], mut index: usize) -> usize {
    while index < v.len()
        && (v[index] == b' ' || v[index] == b'\t' || v[index] == b'\n' || v[index] == b'\r')
    {
        index += 1;
    }
    index
}

pub fn find_next_whitespace(v: &[u8], mut index: usize) -> usize {
    while index < v.len()
        && v[index] != b' '
        && v[index] != b'\t'
        && v[index] != b'\n'
        && v[index] != b'\r'
    {
        index += 1;
    }
    index
}
