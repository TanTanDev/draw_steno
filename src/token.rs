use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)] // I want json naming convention for enum to be all upper case
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, EnumString, EnumIter, AsRefStr)]
#[strum(serialize_all = "lowercase")]
// Important: declaration order matters when parsing   
// EXAMPLE 1:
// if the 'B' token is declared before the 'BR' token
// then the B will first be found resulting in [B, R] instead of [BR] 
// EXAMPLE 2:
// if STENOGRAFI was last declared, We would get [S,T,E,N,O,G,R,A,F,I] instead of [STENOGRAFI] 
pub enum Token {
    // special
    BR,
    STENOGRAFI,

    // Vowels
    A, E, I, O, U, Y,
    #[strum(serialize="å")]
    AO, // å
    #[strum(serialize="ä")]
    AE, // ä
    #[strum(serialize="ö")]
    OO, // ö

    // consonants
    B, C, D, F, G,
    H, J, K, L, M,
    N, P, Q, R, S,
    T, V, W, X, Z,
}
