/*!
This module contains code related to template support.
*/
use crate::error::{MainError, MainResult};
use regex::Regex;
use std::collections::HashMap;

pub fn expand(src: &str, subs: &HashMap<&str, Vec<String>>) -> MainResult<String> {
    let re_sub = Regex::new(r#"#\{([A-Za-z_][A-Za-z0-9_]*)}"#).unwrap();

    // The estimate of final size is the sum of the size of all the input.
    let sub_size = subs.iter().map(|(_, v)| v.iter().map(|v| v.len()).sum::<usize>()).sum::<usize>();
    let est_size = src.len() + sub_size;

    let mut anchor = 0;
    let mut result = String::with_capacity(est_size);

    for m in re_sub.captures_iter(src) {
        // Concatenate the static bit just before the match.
        let (m_start, m_end) = {
            let m_0 = m.get(0).unwrap();
            (m_0.start(), m_0.end())
        };
        let prior_slice = anchor..m_start;
        anchor = m_end;
        result.push_str(&src[prior_slice]);

        // Concat the substitution.
        let sub_name = m.get(1).unwrap().as_str();
        match subs.get(sub_name) {
            Some(srcs) => {
                for src in srcs {
                    result.push_str(src.as_str())
                }
            },
            None => {
                return Err(MainError::OtherOwned(format!(
                    "substitution `{}` in template is unknown",
                    sub_name
                )))
            }
        }
    }
    result.push_str(&src[anchor..]);
    Ok(result)
}
