pub mod ndnf {
    pub mod rpm {
        pub mod nevra {
            use std::cmp::Ordering;

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
                    &self.to_string().unwrap() == &other.to_string().unwrap()
                }
            }

            impl PartialOrd for NEVRA {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    let default_epoch = "0".to_string();

                    if &self == &other {
                        return Some(Ordering::Equal);
                    }

                    if &self.name != &other.name {
                        return None;
                    }

                    if &self.epoch.as_ref().unwrap_or(&default_epoch)
                        < &other.epoch.as_ref().unwrap_or(&default_epoch)
                    {
                        return Some(Ordering::Less);
                    }

                    if &self.epoch.as_ref().unwrap_or(&default_epoch)
                        > &other.epoch.as_ref().unwrap_or(&default_epoch)
                    {
                        return Some(Ordering::Greater);
                    }

                    if &self.release < &other.release {
                        return Some(Ordering::Less);
                    }

                    if &self.release > &other.release {
                        return Some(Ordering::Greater);
                    }

                    None
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
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                        .to_string()
                        .unwrap(),
                        "name-0:version-release.architecture".to_string()
                    );

                    assert_eq!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: Some("1".to_string()),
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                        .to_string()
                        .unwrap(),
                        "name-1:version-release.architecture".to_string()
                    );
                }

                #[test]
                fn NEVRA_compare_eq() {
                    assert_eq!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        },
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_eq_default_epoch() {
                    assert_eq!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: Some("0".to_string()),
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        },
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_name() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        } != NEVRA {
                            name: "other-name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_version() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        } != NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "other-version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_release() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        } != NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "other-release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_neq_architecture() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        } != NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "other-architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_gt_epoch() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: Some("1".to_string()),
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        } > NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_gt_release() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "2".to_string(),
                            architecture: "architecture".to_string(),
                        } > NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "1".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }


                #[test]
                fn NEVRA_compare_lt_epoch() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        } < NEVRA {
                            name: "name".to_string(),
                            epoch: Some("1".to_string()),
                            version: "version".to_string(),
                            release: "release".to_string(),
                            architecture: "architecture".to_string(),
                        }
                    );
                }

                #[test]
                fn NEVRA_compare_lt_release() {
                    assert!(
                        NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "1".to_string(),
                            architecture: "architecture".to_string(),
                        } < NEVRA {
                            name: "name".to_string(),
                            epoch: None,
                            version: "version".to_string(),
                            release: "2".to_string(),
                            architecture: "architecture".to_string(),
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
