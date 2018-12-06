/*  Library for the Zia programming language.
    Copyright (C) 2018  Charles Johnson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
use constants::LABEL;
use traits::{GetDefinition, Id};

pub trait Label<T>
where
    T: GetDefinition<T> + GetNormalFormOf<T> + Clone + Id,
    Self: GetNormalFormOf<T>,
{
    fn get_labellee(&self) -> Option<T> {
        let mut candidates: Vec<T> = Vec::new();
        for label in self.get_normal_form_of() {
            match label.get_definition() {
                None => continue,
                Some((r, x)) => {
                    if r.get_id() == LABEL {
                        candidates.push(x)
                    } else {
                        continue;
                    }
                }
            };
        }
        match candidates.len() {
            0 => None,
            1 => Some(candidates[0].clone()),
            _ => panic!("Multiple concepts are labelled with the same string"),
        }
    }
}

impl<S, T> Label<T> for S
where
    T: GetDefinition<T> + GetNormalFormOf<T> + Clone + Id,
    S: GetNormalFormOf<T>,
{
}

pub trait GetNormalFormOf<T> {
    fn get_normal_form_of(&self) -> Vec<T>;
}
