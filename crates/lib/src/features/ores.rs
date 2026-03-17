use pastey::paste;
use serde::{Deserialize, Serialize};

macro_rules! define_resources {
    (
        $(  $resourceType:ident => [
            $(
                $resource:ident($($props:expr), +$(,)?) => [ $( $ore:ident), + $(,)? ]
            ), + $(,)?
        ]), + $(,)?
    ) => {

        #[derive(Debug, Clone)]
        pub struct MetalProperties {
            pub tier: i8,
            pub smelting_temp: i16,
        }

        impl From<(i8, i16,)> for MetalProperties {
            fn from(value: (i8, i16)) -> Self {
                let tier = value.0;
                let smelting_temp = value.1;
                Self {
                    tier,
                    smelting_temp
                }
            }
        }

        #[derive(Debug, Clone)]
                pub struct FuelProperties {
                    pub burning_temp: i16
                }

        impl From<(i16,)> for FuelProperties {
            fn from(value: (i16,)) -> Self {
                let burning_temp = value.0;
                Self {
                    burning_temp
                }
            }
        }

        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub enum Resource { // metal / fuel
            $( $resourceType($resourceType) ),+
        }

        $(
            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub enum $resourceType {
                $( $resource ), +
            }

            impl $resourceType {
                pub fn ores(&self) -> &[Ore] {
                    match self {
                        $($resourceType::$resource => &[ $( Ore::$ore, )+ ], )+
                    }
                }

                paste! {
                    pub fn properties(&self) -> [<$resourceType Properties>] {
                        match self {
                            $($resourceType::$resource => [<$resourceType Properties>]::from(($($props,)+)), )+
                        }
                    }
                }
            }
        )+

        pub enum Ore {
            $($($( $ore, )+)+)+
        }

        impl Ore {
            pub fn resource(&self) -> Resource {
                match self {
                    $($($(Ore::$ore => Resource::$resourceType( $resourceType::$resource ), )+)+)+
                }
            }
        }
    };
}

define_resources! {
    Fuel => [
        Coal(1200) => [
            Lignite,
            BituminousCoal,
            Anthracite,
        ],
    ],
    Metal => [
        Copper(2, 1084) => [
            NativeCopper,
            Malachite,
        ],
        Iron(4, 1482) => [
            Hematite,
            Magnetite,
            Limonite,
        ],
        MeteoricIron(4, 1476) => [
            Meteor,
        ],
        Gold(2, 1063) => [
            NativeGold,
        ],
        Silver(2, 961) => [
            NativeSilver,
        ],
        Lead(1, 327) => [
            Galena,
        ],
        Tin(1, 232) => [
            Cassiterite,
        ],
        Zinc(1, 419) => [
            Sphalerite,
        ],
        Bismuth(1, 271) => [
            Bismuthinite,
        ],
        Titanium(1, 1668) => [
            Ilmenite,
        ],
        Nickel(1, 1325) => [
            Pentlandite,
        ],
    ],
}
