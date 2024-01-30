use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub iterations: Option<usize>,
    pub depth: usize,
    pub fen: String,
}

pub struct TestSuite(pub Vec<TestCase>);

impl TestSuite {
    pub fn default() -> Self {
        TestSuite(vec![
            TestCase {
                iterations: None,
                depth: 5,
                fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            },
            TestCase {
                iterations: None,
                depth: 4,
                fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - "
                    .to_string(),
            },
            TestCase {
                iterations: None,
                depth: 5,
                fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ".to_string(),
            },
            TestCase {
                iterations: None,
                depth: 5,
                fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
            },
            TestCase {
                iterations: None,
                depth: 4,
                fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string(),
            },
            TestCase {
                iterations: None,
                depth: 4,
                fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 1"
                    .to_string(),
            },
        ])
    }

    pub fn parse_csv(path: PathBuf) -> Result<TestSuite, Box<dyn Error>> {
        let mut cases = Vec::new();
        let mut rdr = csv::Reader::from_path(path)?;
        for case in rdr.deserialize() {
            let case: TestCase = case?;
            cases.push(case);
        }
        Ok(TestSuite(cases))
    }
}
