pub mod ndnf {
    pub mod rpm {
        pub mod nevra {
            #[derive(Debug)]
            enum ParseError {}

            #[derive(Debug)]
            pub struct NEVRA {
                name: String,
                epoch: Option<String>,
                version: String,
                release: String,
                architecture: String,
            }

            impl NEVRA {
                fn from_string(data: String) -> Result<NEVRA, ParseError> {
                    Ok(NEVRA {
                        name: "foo".to_string(),
                        epoch: Some("foo".to_string()),
                        version: "foo".to_string(),
                        release: "foo".to_string(),
                        architecture: "foo".to_string(),
                    })
                }

                fn to_string(&self) -> Result<String, ParseError> {
                    let default_epoch = "0".to_string();

                    let version = &self.version;
                    let architecture = &self.architecture;

                    let mut output = String::new();

                    output = output + &self.name;
                    output = output
                        + &format!("-{}:", &self.epoch.as_ref().unwrap_or(&default_epoch))
                            .to_string();
                    output = output + &self.version;
                    output.push_str("-");
                    output = output + &self.release;
                    output.push_str(".");
                    output = output + &self.architecture;

                    Ok(output)
                }
            }

            impl PartialEq for NEVRA {
                fn eq(&self, other: &Self) -> bool {
                    return self.to_string().unwrap() == other.to_string().unwrap();
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
                            epoch: Some("foo".to_string()),
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                        .to_string()
                        .unwrap(),
                        "foo-foo:foo-foo.foo".to_string()
                    );

                    assert_eq!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                        .to_string()
                        .unwrap(),
                        "foo-0:foo-foo.foo".to_string()
                    )
                }

                #[test]
                fn NEVRA_compare_eq() {
                    assert_eq!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        },
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_name() {
                    assert!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        } != NEVRA {
                            name: "bar".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_eq_default_epoch() {
                    assert_eq!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: Some("0".to_string()),
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        },
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_version() {
                    assert!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        } != NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "bar".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_release() {
                    assert!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        } != NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "bar".to_string(),
                            architecture: "foo".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_architecture() {
                    assert!(
                        NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "foo".to_string(),
                        } != NEVRA {
                            name: "foo".to_string(),
                            epoch: None,
                            version: "foo".to_string(),
                            release: "foo".to_string(),
                            architecture: "bar".to_string(),
                        }
                    );
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
