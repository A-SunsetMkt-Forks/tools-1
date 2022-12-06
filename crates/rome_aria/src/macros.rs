#[macro_export]
macro_rules! define_role {
    ( $( #[doc = $doc:literal] )+ $id:ident {
        PROPS: $p_value:expr,
        ROLES: $r_value:expr,
    }) => {
        $( #[doc = $doc] )*
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPS: &[(&'static str, bool)] = &$p_value;
            const ROLES: &[&'static str] = &$r_value;
        }

        impl $crate::AriaRoleDefinition for $id {
            fn properties<'a>(&self) -> std::slice::Iter<'a, (&str, bool)> {
                $id::PROPS.iter()
            }

            fn roles<'a>(&self) -> std::slice::Iter<'a, &str> {
                $id::ROLES.iter()
            }
        }
    };
    ( $( #[doc = $doc:literal] )+ $id:ident {
        PROPS: $p_value:expr,
        ROLES: $r_value:expr,
        CONCEPTS: $c_value:expr,
    }) => {
        $( #[doc = $doc] )*
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPS: &[(&'static str, bool)] = &$p_value;
            const ROLES: &[&'static str] = &$r_value;
            const CONCEPTS: &'static [(&'static str, &'static [(&'static str, &'static str)])] =
                $c_value;
        }

        impl $crate::AriaRoleDefinition for $id {
            fn properties<'a>(&self) -> std::slice::Iter<'a, (&str, bool)> {
                $id::PROPS.iter()
            }

            fn roles<'a>(&self) -> std::slice::Iter<'a, &str> {
                $id::ROLES.iter()
            }
        }

        impl AriaRoleDefinitionWithConcepts for $id {
            fn concepts_by_element_name<'a>(
                &self,
                element_name: &str,
            ) -> ElementsAndAttributes<'a> {
                for (concept_name, _attributes) in Self::CONCEPTS {
                    if *concept_name == element_name {
                        return Some(Self::CONCEPTS.iter());
                    }
                }
                None
            }
        }
    };
}
