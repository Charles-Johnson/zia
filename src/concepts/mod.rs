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
mod abstract_concept;
mod concrete_concept;
mod string_concept;

use errors::{ZiaError, ZiaResult};
pub use self::abstract_concept::AbstractConcept;
pub use self::concrete_concept::ConcreteConcept;
use self::string_concept::StringConcept;
use reading::{ConcreteReader, GetDefinition, GetReduction, MaybeString};
use writing::{ConcreteWriter, RemoveDefinition, RemoveReduction, SetDefinition, SetReduction};

/// All the different types of concepts.
pub enum Concept {
    /// An abstract concept can reduce to any other concept (whose normal form isn't the former
    /// concept) and can be defined as the composition of any two concepts.
    Abstract(AbstractConcept<ConcreteConcept>),
	/// A concrete concept cannot be further reduced or defined as a composition.
    Concrete(ConcreteConcept),
    /// A string concept is concrete and is associated with a `String` value by the `MaybeString` trait.
    String(StringConcept<ConcreteConcept>),
}

impl From<AbstractConcept<ConcreteConcept>> for Concept {
    fn from(ac: AbstractConcept<ConcreteConcept>) -> Concept {
        Concept::Abstract(ac)
    }
}

impl From<ConcreteConcept> for Concept {
    fn from(cc: ConcreteConcept) -> Concept {
        Concept::Concrete(cc)
    }
}

impl ConcreteReader for Concept {
    type C = ConcreteConcept;
	/// Returns a reference to the concrete part of the concept
    fn read_concrete(&self) -> &ConcreteConcept {
        match *self {
            Concept::Abstract(ref c) => c.read_concrete(),
            Concept::Concrete(ref c) => c,
            Concept::String(ref c) => c.read_concrete(),
        }
    }
}

impl ConcreteWriter for Concept {
    type C = ConcreteConcept;
	/// Returns a mutable reference to the concrete part of the concept
    fn write_concrete(&mut self) -> &mut ConcreteConcept {
        match *self {
            Concept::Abstract(ref mut c) => c.write_concrete(),
            Concept::Concrete(ref mut c) => c,
            Concept::String(ref mut c) => c.write_concrete(),
        }
    }
}

impl GetDefinition for Concept {
	/// If concept is abstract and has a definition returns the indices of the left and right concepts that compose it as `Some((left, right))`. Otherwise returns `None`.
    fn get_definition(&self) -> Option<(usize, usize)> {
        match *self {
            Concept::Abstract(ref c) => c.get_definition(),
            Concept::String(_) => None,
            Concept::Concrete(_) => None,
        }
    }
}

impl SetDefinition for Concept {
	/// Sets the definition of the concept if abstract, otherwise returns an error.
    fn set_definition(&mut self, lefthand: usize, righthand: usize) -> ZiaResult<()> {
        match *self {
            Concept::Abstract(ref mut c) => c.set_definition(lefthand, righthand),
            _ => Err(ZiaError::SettingDefinitionOfConcrete),
        }
    }
}

impl RemoveDefinition for Concept {
	/// Removes the definition of the concept if abstract, otherwise panics.
    fn remove_definition(&mut self) {
        match *self {
            Concept::Abstract(ref mut c) => c.remove_definition(),
            Concept::String(_) => panic!("String concepts do not have a definition to remove"),
            Concept::Concrete(_) => panic!("Concrete concepts do not have a definition to remove"),
        }
    }
}

impl GetReduction for Concept {
	/// Gets the index of the concept that `self` may reduce to.
    fn get_reduction(&self) -> Option<usize> {
        match *self {
            Concept::Abstract(ref c) => c.get_reduction(),
            Concept::String(_) => None,
            Concept::Concrete(_) => None,
        }
    }
}

impl SetReduction for Concept {
	/// Sets the index of the concept that `self` reduces to if abstract. Otherwise returns an error.
    fn make_reduce_to(&mut self, concept: usize) -> ZiaResult<()> {
        match *self {
            Concept::Abstract(ref mut c) => c.make_reduce_to(concept),
            _ => Err(ZiaError::ConcreteReduction),
        }
    }
}

impl RemoveReduction for Concept {
	/// Removes the reduction rule of the concept if abstract, otherwise panics.
    fn make_reduce_to_none(&mut self) {
        match *self {
            Concept::Abstract(ref mut c) => c.make_reduce_to_none(),
            Concept::String(_) => panic!("String concepts have no reduction rule to remove"),
            Concept::Concrete(_) => panic!("Concrete concepts have no reduction rule to remove"),
        };
    }
}

impl From<String> for Concept {
    fn from(string: String) -> Concept {
        Concept::String(string.into())
    }
}

impl MaybeString for Concept {
	/// Gets the `String` value associated with `self` if it is a string concept. Otherwise returns `None`.
    fn get_string(&self) -> Option<String> {
        match *self {
            Concept::String(ref s) => s.get_string(),
            _ => None,
        }
    }
}
