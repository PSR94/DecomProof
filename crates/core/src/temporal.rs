use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct PatternInference { pub pattern:String, pub confidence:String, pub median_gap_days:i64, pub recommended_window_days:u32 }

pub fn infer_pattern(mut observations:Vec<DateTime<Utc>>)->Option<PatternInference>{
    if observations.len()<2{return None} observations.sort();
    let mut gaps:Vec<i64>=observations.windows(2).map(|w|(w[1]-w[0]).num_days().abs()).collect(); gaps.sort(); let median=gaps[gaps.len()/2];
    let (pattern,recommended)=match median {0..=2=>("daily",7),3..=10=>("weekly",21),11..=45=>("monthly",90),46..=120=>("approximately-quarterly",180),_=>("rare-or-irregular",(median*2).clamp(30,730) as u32)};
    Some(PatternInference{pattern:pattern.into(),confidence:if gaps.len()>=3{"medium".into()}else{"low".into()},median_gap_days:median,recommended_window_days:recommended})
}

#[cfg(test)] mod tests {use super::*; use chrono::TimeZone; #[test] fn quarterly_pattern_is_conservative(){let a=Utc.with_ymd_and_hms(2026,3,31,0,0,0).unwrap();let b=Utc.with_ymd_and_hms(2026,6,30,0,0,0).unwrap();let p=infer_pattern(vec![a,b]).unwrap();assert_eq!(p.pattern,"approximately-quarterly");assert_eq!(p.confidence,"low");}}
