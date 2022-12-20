
pub mod soypricing {
	std::fs;
	fn get_path_from_directory() {
		let dir= fs::read_dir().unwrap();
		for pth in dir {
			println!("Name: {}", path.unwrap().path().display());
		}

	}

	pub mod fetchdata {
    use dbz_lib::{Dbz, DbzStreamIter};
    use std::time::Duration;
    use std::io::Cursor;
    use suppaftp::FtpStream;
    pub fn connect_ftp(url_name: &str, ext: &str) {
        let mut ftp_stream = FtpStream::connect(url_name).unwrap();
        let _ = ftp_stream.login("wfrancis@fordham.edu", "Lebronjames1").unwrap();
        println!("Current directory: {}", ftp_stream.pwd().unwrap());
        let _ = ftp_stream.cwd(ext).unwrap();
        println!("Current directory: {}", ftp_stream.pwd().unwrap());
        let fl1 = ftp_stream.retr_as_buffer("/GLBX-20221130-VPX9AXX459/glbx-mdp3-20220629.trades.dbz").unwrap();
        println!("{:?}", fl1.schema());
        //println!("{:?}",fl1.into_inner());
        //println!("{:?}",ftp_stream.list(Some("/GLBX-20221130-VPX9AXX459/")));
    }
}

}
