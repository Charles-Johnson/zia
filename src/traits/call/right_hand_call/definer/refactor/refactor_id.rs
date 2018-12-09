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
use traits::call::right_hand_call::definer::ConceptNumber;
use traits::GetId;

pub trait ConceptTidyer<T> {
    fn remove_concept(&mut self, &T);
    fn correct_id(&mut self, usize);
}

pub trait ConceptCleaner<T>
where
    Self: ConceptTidyer<T> + ConceptNumber,
    T: GetId,
{
    fn cleanly_remove_concept(&mut self, concept: &T) {
        self.remove_concept(concept);
        for id in concept.get_id()..self.number_of_concepts() {
            self.correct_id(id);
        }
    }
}

impl<S, T> ConceptCleaner<T> for S
where
    S: ConceptTidyer<T> + ConceptNumber,
    T: GetId,
{
}
