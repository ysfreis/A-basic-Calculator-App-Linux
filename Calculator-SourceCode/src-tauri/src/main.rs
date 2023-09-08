// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//A basic Calculator

//Powerden By ysk.dev
//https://www.apache.org/licenses/LICENSE-2.0

#[tauri::command]
fn hesapla(a: &str,b: &str,c: &str) -> String {
    let sayi = match a.parse::<f64>() {
        Ok(deger) => deger,
        Err(_) => return format!("Hatalı giriş: {} veya {}", a, b),
    };
    let sayi2 = match c.parse::<f64>() {
        Ok(deger) => deger,
        Err(_) => return format!("Hatalı giriş: {} veya {}", a, b),
    };
    let sonuc = if b=="+"{
        sayi+sayi2
    }else if b=="-"{
        sayi-sayi2
         
    }else if b=="/"{
        sayi/sayi2
    }else{
        sayi*sayi2

    };
    
    println!("{}",sonuc);
    
    format!("Yapılan İşlem:{}{}{}       Sonuç:{} ", a,b,c,sonuc)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hesapla])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
//iletişim:discord:ysk.dev
