use std::path::Path;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::{Write, Result as IoResult,self, BufWriter,IoSlice};
use memmap::MmapMut;
use rayon::prelude::*;
use bincode;

// i5 10th gen, 8gb, 4c
// total record : 18000 => 1.2kb => 1.78 mb
// time taken ms || write speed i/s

// 8.57 || 2,098,000 

pub fn write_record(file_path: &str, records: Vec<(String, String)>) -> IoResult<()> {
    let start_time = Instant::now(); 

    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)?;
    
    let mut buffer = String::new();
    for (header, body) in &records {
        let combined = format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header); 
        buffer.push_str(&combined);
    }
    
    file.write_all(buffer.as_bytes())?;
    
    let duration = start_time.elapsed(); 

    println!("time taken: {:?}", duration); 
    println!("written i: {}", records.len()); 
    println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64()); 

    Ok(())
}

// 36.2 || 496,000 

pub fn write_record2(file_path: &str, records: Vec<(String, String)>) -> IoResult<()> {
    let start_time = Instant::now();
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)?;

    let mut buffer = Vec::new();

    for (header, body) in &records {
        let record = ("2024-07-20 20:10:44.237606774 UTC", body, header);
        let serialized = bincode::serialize(&record)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        buffer.extend_from_slice(&serialized);

    }

    file.write_all(&buffer)?;

    let duration = start_time.elapsed(); 

    println!("time taken: {:?}", duration); 
    println!("written i: {}", records.len()); 
    println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64()); 

    Ok(())
}

// 25.7 || 700,000

pub fn write_record3(file_path: &str, records: Vec<(String, String)>) -> IoResult<()> {
    let start_time = Instant::now();

    let path = Path::new(file_path);
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    let mut file_len = file.metadata()?.len();

    if file_len == 0 {
        file.set_len(1)?;
        file_len = 1;
    }

    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    let mut buffer = String::new();
    for (header, body) in &records {
        let combined = format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header);
        buffer.push_str(&combined);
    }

    let buffer_bytes = buffer.as_bytes();
    let new_len = file_len + buffer_bytes.len() as u64;

    
    if new_len > mmap.len() as u64 {
        
        file.set_len(new_len)?;
        
        mmap = unsafe { MmapMut::map_mut(&file)? };
    }

    
    let start_offset = file_len as usize;
    let mmap_mut = unsafe { mmap.as_mut() };
    mmap_mut[start_offset..start_offset + buffer_bytes.len()].copy_from_slice(buffer_bytes);

    
    mmap.flush()?;

    let duration = start_time.elapsed();
    println!("time taken: {:?}", duration);
    println!("written i: {}", records.len());
    println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64());

    Ok(())
}

// 13.44 || 1,339,000 
pub fn write_record4(file_path: &str, records: Vec<(String, String)>) -> io::Result<()> {
    let start_time = Instant::now();

    let file = OpenOptions::new()
        .append(true)
        .open(file_path)?;
    
    // deafult
    let mut buffer = BufWriter::new(file);
    let mut content = String::new();

    for (header, body) in &records {
        let combined = format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header);
        content.push_str(&combined);
    }

    buffer.write_all(content.as_bytes())?;
    
    //  method 2
    // let mut buffer = BufWriter::new(file);

    // for (header, body) in &records {
    //     let combined = format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header);
    //     buffer.write(combined.as_bytes())?;
    // }

    buffer.flush()?;
    let duration = start_time.elapsed();

    println!("time taken: {:?}", duration);
    println!("written i: {}", records.len());
    println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64());

    Ok(())
}

// 12.27 || 1,465,000

pub fn write_record5(file_path: &str, records: Vec<(String, String)>) -> io::Result<()> {
        let start_time = Instant::now();
    
        let file = OpenOptions::new()
            .append(true)
            .open(file_path)?;
    
        let mut buffer = BufWriter::new(file);
        let mut iovec = Vec::with_capacity(records.len());
        let mut combined_strings = Vec::with_capacity(records.len()); 
    
        for (header, body) in &records {
            let combined = format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header);
            combined_strings.push(combined);
        }
    
        for combined in &combined_strings {
            iovec.push(IoSlice::new(combined.as_bytes()));
        }
    
        buffer.write_vectored(&iovec)?;
        buffer.flush()?;
    
        let duration = start_time.elapsed();
    
        println!("time taken: {:?}", duration);
        println!("written i: {}", records.len());
        println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64());
    
        Ok(())
}    

// 7.38 || 2,436,000

pub fn write_record6(file_path: &str, records: Vec<(String, String)>) -> io::Result<()> {
    let start_time = Instant::now();

    let file = OpenOptions::new()
        .append(true)
        .open(file_path)?;

    let mut buffer = BufWriter::new(file);
    
    let combined_strings: Vec<String> = records.par_iter()
        .map(|(header, body)| {
            format!("{} {} {}\n", "2024-07-20 20:10:44.237606774 UTC", body, header)
        })
        .collect();

    let iovec: Vec<IoSlice> = combined_strings.par_iter()
        .map(|combined| IoSlice::new(combined.as_bytes()))
        .collect();

    buffer.write_vectored(&iovec)?;
    buffer.flush()?;

    let duration = start_time.elapsed();

    println!("time taken: {:?}", duration);
    println!("written i: {}", records.len());
    println!("write speed: {} i/s", records.len() as f64 / duration.as_secs_f64());

    Ok(())
}