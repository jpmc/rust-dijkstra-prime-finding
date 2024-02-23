pub mod dijkstra_naive;
pub mod dijkstra_attempt_1;
pub mod dijkstra_attempt_2;
pub mod dijkstra_attempt_3;



// A pretty-printer for Vec<(u32, u32)> results.
// Specifically meant for just printing the first u32 in the tuple, as a comma-delimited string.
#[allow(dead_code)]
fn print_vec2(v: Vec<(u64, u64)>) {
    println!("{}", v.iter()
        .map(|t| t.0.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
}
#[allow(dead_code)]
fn print_vec3(v: Vec<(u64, u64, u64)>) {
    println!("{}", v.iter()
        .map(|t| t.0.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    );
}

// Generic version from ChatGPT that simply disregards the tuple structure
// and just says "first value must be printable".
// It perfectly replicates my manual attempt but in a more generic manner.
#[allow(dead_code)]
fn print_vec_gpt<T, U>(v: Vec<(T, U)>)
    where T: std::fmt::Display,
{
    let result = v
        .iter()
        .map(|(first, _)| first.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", result);
}