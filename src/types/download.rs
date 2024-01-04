use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub(crate) fn download_audio() {
    let client = reqwest::blocking::Client::new();

    let text = "Wie geht es Ihnen?";
    let x_amz_date = "20240103T165407Z";
    let x_amz_security_token = "IQoJb3JpZ2luX2VjEMn//////////wEaCXVzLWVhc3QtMSJHMEUCIQCbbM8AWCsgRq8lVXAWhUdQFVCaxGIvGz9/qavELo/3cQIgJURKkijwlD6GmFsKetWXE9C7EqErPnJV/iXE4KyolZEqyQUIYhAAGgwwMDUwNTI0NjAwMTUiDOF6sLhdmxXfqJXjciqmBVvO3Zwt7Q5e/zzywLyYa/vAV/LFDx6IXMRyMESYsCYzHY2QjNPt/muU5B4fQEHAB6K8wd7kCHpzM6BOE6BMF1/Naw3ZNVHftqnWmVX+uGprTEu6jVuxV+flZLdxf3lDHmkcMq1XT3y5X6BXiRuT/uE6ZyGVm0HOQuqoxNi6otRk/+5ctfTKMWC8FLVDnuqd4JN+GceLtX9PlFfKThKjqL+sG9Mnj1zcs9wqlPOOJup81w+e0j8hI1iH7drf5bl0Gm6xW9/wI7I2faw5KMq+eywIcUqZbdzBMJQKa6yANC3S30AopmId8NosFFX55NGTWjub0OMHP4yopkPiXvbVsNxVOqGZMmGQsEzdb2l/f4ddw3VdDvoCAeOoVmWxLG0fH8xTGU6dyFYPiCqV4GFk5CqRtU+nZSoDDmrEQKTVWkk4h4enkLwoesBpNnWtjaUK+DPhnJsMtDnJblLRuZ/uV8LLwUJvOJsbCDJF+KGFaiWNtkfu6s3A8kwemRfIst/Ovub3JSqC21t/0uuh0nBaHWd8tdehbNcA1b9FKmbMz9EJkdPmnM6yjMmnhG4wRrSWteF/fV33eRoW5pUOjT4sj0P6bHY+tMGK3GdHjmfiYU9RXoS7gN2bOAz9qIzOssqBbylRPmnTw+39Ju1nlpmrom4r/dKq+hZknZdHo2df9plaxgQBiJg7sk5/sxncIYDUdK1zFv5hPoPz54P++WUCqvygqDOf+ydNgHQKFbRuvz1ddVI0bkgU2B5mEgCE/3zEqGmSe9GGaDvvmJhF3Sfeae9aseJ+7PvqejH4/gfJFkSdLXt5irXrc7oqlWbulpFJdZhcj0oIkzNmCeUxvrq5kJak/UOLyfa2RFRhsuq+TxrCyFjTVkOAYi5rY79tmZez/cqen//tYjDxn9asBjrdArA+2JZyrjy4xDawwo6LOU3qx310h33UEB4moTr61VJ0dSo6TIbXeVe030DTmHna6k5S7WEpwe5W2jfofWpzgV33xdvGm57+Qt03Sb/zyFG3Q2UlZA0Glhh/uZOiHBYtBirCH7yDwHjWEbDGZ9S7dpdGriXOxvooSaGBCE/qLSTAM57g3j3V//k7ILYOwC+PuqnuqKPNyIJTTRRgaNEfVj1yyKrVCEOxuPEs/bf9gV9U68LQp/O1wqtuacwnovyHfnrLg+1CH/Sqmgc17LaQW+EX1FGxT91AO8nvebRS2dfjWXNYqjWdm/bc8A+45L47jxG06AjFKu1rL2jPCr7keWCOJOrkh+fxFcTpSwpvMPmqi/quRtO6PNn1ICE84/oLkicnEl1EWaQSF5Y5zpHIAvFjmN5mhoSnput70p/ypGv3+irxyIFJHy3Ok20oPwqJTMMrwBEyS613diHRRy4=";
    let authorization = "AWS4-HMAC-SHA256 Credential=ASIAQCLJGNPXTHK53YT3/20240103/us-east-1/execute-api/aws4_request, SignedHeaders=content-type;host;x-amz-date;x-amz-security-token, Signature=7e287f4bcd2ca0781c2ae0b74baa73bc476aa49a9ed07bc958cdd45bdbcab9cd";

    let response =   client.post("https://2poo4vxwjc.execute-api.us-east-1.amazonaws.com/prod-wps/tts?e=user%40naturalreaders.com&l=0&r=29&s=0&v=ms&vn=10.3.1&sm=false&ca=false")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Content-Type", "application/json; charset=UTF-8")
        .header("Referer", "https://www.naturalreaders.com/")
        .header("x-amz-date", x_amz_date)
        .header("X-Amz-Security-Token", x_amz_security_token)
        .header("Origin", "https://www.naturalreaders.com")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "cross-site")
        .header("Authorization", authorization)
        .header("Connection", "keep-alive")
        .body(format!(r#"{{"t":"{}"}}"#, text))
        .send()
        .unwrap();

    println!("Status: {}", response.status());

    if response.status().is_success() {
        let mut content = Cursor::new(response.bytes().unwrap());
        let path = PathBuf::from("audio/phrases")
            .with_file_name(text)
            .with_extension("mp3")
            .as_path();
        let mut file = File::create(path).unwrap();
        io::copy(&mut content, &mut file).unwrap();
    }
}

pub(crate) fn download_file(link: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Downloading audio from {} to {}", link, path.display());

    let response = reqwest::blocking::get(link)?;

    if response.status().is_success() {
        let mut content = Cursor::new(response.bytes()?);
        let mut file = File::create(path)?;
        create_parent_directory_if_missing(path)?;
        io::copy(&mut content, &mut file)?;
        Ok(())
    } else {
        Err(Box::from("Failed to download audio file"))
    }
}

fn create_parent_directory_if_missing(path: &Path) -> Result<(), Box<dyn Error>> {
    match path.parent() {
        Some(parent) => fs::create_dir_all(parent)?,
        None => {}
    };

    Ok(())
}
