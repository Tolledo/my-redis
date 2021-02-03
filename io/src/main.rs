use tokio::fs::{remove_file, File};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let file_name = "foo.txt";

    let mut file = File::create(file_name).await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes is all I got").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);

    let mut f = File::open(file_name).await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    println!("First 10 bytes bytes: {:?}", &buffer[..n]);

    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    println!("The rest of file: {:?}", &buffer);

    let mut reader: &[u8] = b"hello";
    let mut file = File::create(file_name).await?;

    // asynchronously copy the entire content of a reader into a writer
    io::copy(&mut reader, &mut file).await?;

    // remove file from filesystem
    remove_file(file_name).await?;

    Ok(())
}
