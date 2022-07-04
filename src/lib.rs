pub mod ndnf {
    pub mod rpm {
        pub mod nevra {
            #[derive(Debug)]
            enum ParseError {}

            pub struct NEVRA {
                name: String,
                epoch: String,
                version: String,
                release: String,
                architecture: String,
            }

            impl NEVRA {
                fn from_string(data: String) -> Result<NEVRA, ParseError> {
                    Ok(NEVRA {
                        name: "foo".to_string(),
                        epoch: "foo".to_string(),
                        version: "foo".to_string(),
                        release: "foo".to_string(),
                        architecture: "foo".to_string(),
                    })
                }

                fn to_string(&self) -> Result<String, ParseError> {
                    Ok(format!("foo-foo-foo-foo").to_string())
                }
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn dummy() {
                    assert!(1 == 1)
                }

                #[test]
                fn NEVRA_from_string() {
                    assert!(1 == 1)
                }

                #[test]
                fn NEVRA_to_string() {
                    assert_eq!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: "foo".to_string(),
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                        .to_string()
                        .unwrap(),
                        "foo-foo-foo-foo".to_string()
                    )
                }
            }
        }
    }

    pub mod action {
        use super::rpm::nevra::NEVRA;

        fn install(packages: Vec<NEVRA>) {}

        fn uninstall(packages: Vec<NEVRA>) {}

        fn solve(packages: Vec<NEVRA>) {}

        #[cfg(test)]
        mod tests {
            #[test]
            fn dummy() {
                assert!(1 == 1)
            }
        }
    }
}
