extern crate sdl2;
macro_rules! struct_events {
    (
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },
        else: { $( $e_alias:ident : $e_sdl:pat),* }
    ) => {
        use sdl2::EventPump;


        pub struct ImmediateEvents {
            $( pub $k_alias: Option<bool>, )*
            $( pub $e_alias: bool, )*
            resize: Option<(u32, u32)>
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    $($k_alias: None,)*
                    $($e_alias: false,)*
                    resize: None
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,
            $( pub $k_alias: bool),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),
                    $( $k_alias: false),*
                }
            }

            pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;
                    use sdl2::event::WindowEventId::Resized;

                    match event {
                        Window { win_event_id: Resized, ..} => {
                            self.now.resize = Some(renderer.output_size().unwrap());
                        },
                        KeyDown {keycode, ..} => match keycode {
                            $(
                                Some($k_sdl) => {
                                    if !self.$k_alias {
                                        self.now.$k_alias = Some(true);
                                    }
                                    self.$k_alias = true;
                                }

                            ),*
                            _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }

                             ),*
                            _ => {}
                        },
                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                        )*,
                        _ => {}
                    }
                }
            }
        }
    }
}