use anyhow::Result;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, XcbDisplayHandle,
    XcbWindowHandle,
};
use x11rb::{
    atom_manager,
    connection::Connection,
    protocol::xproto::{self, ConnectionExt, CreateGCAux, CreateWindowAux, Screen, WindowClass},
    xcb_ffi::XCBConnection,
};

use super::WallpaperSurface;

atom_manager! {
    pub Atoms: AtomsCookie {
        _NET_WM_STATE_ABOVE,
        _NET_WM_STATE_BELOW,
    }
}

pub struct X11Renderer {
    pub conn: XCBConnection,
    pub screen: Screen,
    pub pixmap: u32,
    pub win_id: u32,
    pub context: u32,
}

impl WallpaperSurface for X11Renderer {
    fn new() -> Result<Self> {
        let (conn, screen_num) = x11rb::xcb_ffi::XCBConnection::connect(None)?;
        let atoms = Atoms::new(&conn)?.reply()?;
        let screen = &conn.setup().roots[screen_num];
        let win_id = conn.generate_id()?;
        conn.create_window(
            screen.root_depth,
            win_id,
            screen.root,
            0,
            0,
            screen.width_in_pixels,
            screen.height_in_pixels,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().override_redirect(1),
        )?;
        let pixmap = conn.generate_id()?;
        conn.create_pixmap(
            screen.root_depth,
            pixmap,
            win_id,
            screen.width_in_pixels,
            screen.height_in_pixels,
        )?;
        let context = conn.generate_id()?;
        let gc_aux = CreateGCAux::new().foreground(screen.white_pixel);
        conn.create_gc(context, win_id, &gc_aux)?;
        conn.map_window(win_id)?;
        conn.configure_window(
            win_id,
            &xproto::ConfigureWindowAux::default().stack_mode(xproto::StackMode::BELOW),
        )?;
        conn.flush()?;
        Ok(X11Renderer {
            screen: conn.setup().roots[screen_num].clone(),
            conn,
            pixmap,
            context,
            win_id,
        })
    }

    fn destroy(&self) -> Result<()> {
        Ok(())
    }

    fn redraw(&self) -> Result<()> {
        // self.conn.poly_fill_rectangle(
        //     self.pixmap,
        //     self.context,
        //     &[Rectangle {
        //         x: 0,
        //         y: 0,
        //         width: 400,
        //         height: 400,
        //     }],
        // )?;
        self.conn.copy_area(
            self.pixmap,
            self.screen.root,
            self.context,
            0,
            0,
            0,
            0,
            self.screen.width_in_pixels,
            self.screen.height_in_pixels,
        )?;
        self.conn.flush()?;
        Ok(())
    }

    fn dimensions(&self) -> (u16, u16) {
        (self.screen.width_in_pixels, self.screen.height_in_pixels)
    }
}

unsafe impl HasRawWindowHandle for X11Renderer {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = XcbWindowHandle::empty();
        handle.window = self.win_id;
        RawWindowHandle::Xcb(handle)
    }
}

unsafe impl HasRawDisplayHandle for X11Renderer {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        let mut handle = XcbDisplayHandle::empty();
        handle.connection = self.conn.get_raw_xcb_connection();
        RawDisplayHandle::Xcb(handle)
    }
}
