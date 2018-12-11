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
use concepts::traits::{GetDefinition, DeleteReduction};
use ast::traits::{Display, MightExpand};
pub use self::insert_definition::InsertDefinition;
use self::syntax_from_concept::{GetLabel, SyntaxFactory, match_left_right, Pair};
pub use self::syntax_from_concept::{Combine, SyntaxFromConcept, MaybeConcept};
use utils::{ZiaError, ZiaResult};

impl<T> MightExpand for T
where
    T: GetDefinition<T>,
{
    fn get_expansion(&self) -> Option<(T, T)> {
        self.get_definition()
    }
}

pub trait TryRemovingReduction<T> 
where
	Self: MaybeConcept<T>,
	T: DeleteReduction,
{
	fn try_removing_reduction(&mut self) -> ZiaResult<()> {
		if let Some(mut c) = self.get_concept() {
            c.delete_reduction();
            Ok(())
        } else {
            Err(ZiaError::RedundantReduction)
        }
	}
}

impl<S, T> TryRemovingReduction<T> for S
where
	S: MaybeConcept<T>,
	T: DeleteReduction,
{}

pub trait Expander<T>
where
    T: Display + SyntaxFromConcept,
    Self: MaybeConcept<T>
        + MightExpand
        + Display
        + Clone
        + Pair<T, Self>
        + Combine<T>
        + SyntaxFactory<T>,
{
    fn expand(&self) -> Self {
        if let Some(ref con) = self.get_concept() {
            if let Some((ref left, ref right)) = con.get_definition() {
                left.to_ast::<Self>()
                    .expand()
                    .combine_with(&right.to_ast::<Self>().expand())
            } else {
                con.to_ast::<Self>()
            }
        } else if let Some((ref left, ref right)) = self.get_expansion() {
            left.expand().combine_with(&right.expand())
        } else {
            self.clone()
        }
    }
}

impl<S, T> Expander<T> for S
where
    T: Display + SyntaxFromConcept,
    S: MaybeConcept<T> + MightExpand + Display + Clone + Pair<T, S> + Combine<T> + SyntaxFactory<T>,
{
}

pub trait Reduce<T>
where
    T: SyntaxFromConcept,
    Self: SyntaxFactory<T> + Combine<T> + MightExpand + Clone,
{
    fn recursively_reduce(&self) -> Self {
        match self.reduce() {
            Some(ref a) => a.recursively_reduce(),
            None => self.clone(),
        }
    }
    fn reduce(&self) -> Option<Self> {
        match self.get_concept() {
            Some(ref c) => c.reduce(),
            None => match self.get_expansion() {
                None => None,
                Some((ref left, ref right)) => {
                    match_left_right::<T, Self>(left.reduce(), right.reduce(), left, right)
                }
            },
        }
    }
}

impl<S, T> Reduce<T> for S
where
    T: SyntaxFromConcept,
    S: SyntaxFactory<T> + Combine<T> + MightExpand + Clone,
{
}

impl<T: GetLabel> Display for T {
	fn to_string(&self) -> String {
	    match self.get_string() {
	        Some(s) => "\"".to_string() + &s + "\"",
	        None => match self.get_label() {
	            Some(l) => l,
	            None => match self.get_definition() {
	                Some((left, right)) => {
	                    let mut left_string = left.to_string();
	                    if left_string.contains(' ') {
	                        left_string = "(".to_string() + &left_string;
	                    }
	                    let mut right_string = right.to_string();
	                    if right_string.contains(' ') {
	                        right_string += ")";
	                    }
	                    left_string + " " + &right_string
	                }
	                None => panic!("Unlabelled concept with no definition!"),
	            },
	        },
	    }
	}
}

mod syntax_from_concept {
	pub use concepts::traits::GetLabel;
	use self::combine::FindDefinition;
	pub use self::combine::{Combine, Pair, MaybeConcept};
	pub use ast::traits::SyntaxFactory;
	pub trait SyntaxFromConcept
	where
		Self: GetLabel + FindDefinition<Self> + PartialEq,
	{
		fn reduce<U: SyntaxFactory<Self> + Combine<Self> + Clone>(&self) -> Option<U> {
		    match self.get_normal_form() {
		        None => match self.get_definition() {
		            Some((ref left, ref right)) => {
		                let left_result = left.reduce();
		                let right_result = right.reduce();
		                match_left_right::<Self, U>(
		                    left_result,
		                    right_result,
		                    &left.to_ast(),
		                    &right.to_ast(),
		                )
		            }
		            None => None,
		        },
		        Some(ref n) => Some(n.to_ast()),
		    }
		}
		fn to_ast<U: SyntaxFactory<Self> + Combine<Self> + Clone>(&self) -> U {
		    match self.get_label() {
		        Some(ref s) => U::new(s, Some(self.clone())),
		        None => match self.get_definition() {
		            Some((ref left, ref right)) => {
		                left.to_ast::<U>().combine_with(&right.to_ast::<U>())
		            }
		            None => panic!("Unlabelled concept with no definition"),
		        },
		    }
		}
	}

	impl<S> SyntaxFromConcept for S where S: GetLabel + FindDefinition<S> + PartialEq {}

	pub fn match_left_right<T: GetLabel + FindDefinition<T> + PartialEq, U: Combine<T>>(
		left: Option<U>,
		right: Option<U>,
		original_left: &U,
		original_right: &U,
	) -> Option<U> {
		match (left, right) {
		    (None, None) => None,
		    (Some(new_left), None) => Some(contract_pair::<T, U>(&new_left, original_right)),
		    (None, Some(new_right)) => Some(contract_pair::<T, U>(original_left, &new_right)),
		    (Some(new_left), Some(new_right)) => Some(contract_pair::<T, U>(&new_left, &new_right)),
		}
	}

	fn contract_pair<T: GetLabel + FindDefinition<T> + PartialEq, U: Combine<T>>(
		lefthand: &U,
		righthand: &U,
	) -> U {
		if let (Some(lc), Some(rc)) = (lefthand.get_concept(), righthand.get_concept()) {
		    if let Some(def) = lc.find_definition(&rc) {
		        if let Some(ref a) = def.get_label() {
		            return U::from_pair(a, Some(def), &lefthand, &righthand);
		        }
		    }
		}
		lefthand.combine_with(righthand)
	}

	mod combine {
		pub use concepts::traits::FindDefinition;
		pub use ast::traits::{MaybeConcept, Pair};
		use ast::traits::DisplayJoint;

		pub trait Combine<T>
		where
			Self: DisplayJoint + MaybeConcept<T> + Pair<T, Self> + Sized,
			T: FindDefinition<T> + Clone + PartialEq,
		{
			fn combine_with(&self, other: &Self) -> Self {
				let left_string = self.display_joint();
				let right_string = other.display_joint();
				let definition = if let (Some(l), Some(r)) = (self.get_concept(), other.get_concept()) {
				    l.find_definition(&r)
				} else {
				    None
				};
				Self::from_pair(
				    &(left_string + " " + &right_string),
				    definition,
				    self,
				    other,
				)
			}
		}

		impl<T, U> Combine<T> for U
		where
			U: DisplayJoint + MaybeConcept<T> + Pair<T, U> + Sized,
			T: FindDefinition<T> + Clone + PartialEq,
		{
		}
	}
}

mod insert_definition {
	use concepts::traits::{SetDefinition, GetReduction};
	use ast::traits::Container;
	use utils::{ZiaError, ZiaResult};

	pub trait InsertDefinition
	where
		Self: SetDefinition<Self> + Sized + Container + GetReduction<Self>,
	{
		fn insert_definition(&mut self, lefthand: &mut Self, righthand: &mut Self) -> ZiaResult<()> {
		    if lefthand.contains(self) || righthand.contains(self) {
		        Err(ZiaError::InfiniteDefinition)
		    } else {
		        try!(self.check_reductions(lefthand));
		        try!(self.check_reductions(righthand));
		        self.set_definition(lefthand, righthand);
		        lefthand.add_as_lefthand_of(self);
		        righthand.add_as_righthand_of(self);
		        Ok(())
		    }
		}
		fn check_reductions(&self, concept: &Self) -> ZiaResult<()> {
		    if let Some(ref r) = concept.get_reduction() {
		        if r == self || r.contains(self) {
		            Err(ZiaError::ExpandingReduction)
		        } else {
		            self.check_reductions(r)
		        }
		    } else {
		        Ok(())
		    }
		}
	}

	impl<T> InsertDefinition for T where
		T: SetDefinition<T> + Sized + Container + GetReduction<Self>
	{
	}
}
