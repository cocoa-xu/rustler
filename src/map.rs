use super::{ NifEnv, NifTerm };
use ::atom::NifAtom;

pub fn get_map_value<'a>(env: &'a NifEnv, term: NifTerm, key: NifTerm) -> Result<NifTerm<'a>, ()> {
    match ::wrapper::get_map_value(env.as_c_arg(), term.as_c_arg(), key.as_c_arg()) {
        Ok(value) => Ok(NifTerm::new(env, value)),
        Err(()) => Err(()),
    }
}

pub fn get_ex_struct_name(env: &NifEnv, map: NifTerm) -> Option<NifAtom> {
    // In an Elixir struct the value in the __STRUCT__ field is always an atom.
    match get_map_value(env, map, ::atom::get_atom_init("__STRUCT__").to_term(env)) {
        Ok(term) => unimplemented!(),
        Err(()) => None,
    }
}