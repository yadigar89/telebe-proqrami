
use std::io;
use std::fs;
struct Telebe{
    ad: String,
    yas: i32,
    ixtisas: String,
}
fn setirden_telebe(setir: &str) ->Result<Telebe, String>{

    let hisseler: Vec<&str> = setir.split(',').collect();

    if hisseler.len() != 3 {
        return Err(String::from("Setirde 3 hisse olmalidir."));
    }
    let yas = hisseler[1]
       .trim()
       .parse::<i32>()
       .map_err(|_| String::from("Yas ededi duz deyil."))?;

    Ok(Telebe{
        ad: String::from(hisseler[0].trim()),
        yas,
        ixtisas: String::from(hisseler[2].trim()),
    })
}
fn fayildan_oxu() ->Vec<Telebe>{
    
    let mut telebeler: Vec<Telebe> = Vec::new();

    match fs::read_to_string("telebeler.txt"){
        Ok(metn) =>{
            for setir in metn.lines(){
                match setirden_telebe(setir){
                    Ok(t) =>telebeler.push(t),
                    Err(e) => println!("Xeta: {e} ->{setir}"),
                }
            }
        }
        Err(_) =>println!("Fayl tapilmadi, bos siyahi ile baslayiriq."),
    }
    telebeler
}
fn fayla_yaz(telebeler: &[Telebe]) ->Result<(), std::io::Error>{

    let mut metn = String::new();

    for t in telebeler{
        metn.push_str(&format!("{}, {}, {}\n",t.ad, t.yas, t.ixtisas));
    }
    fs::write("telebeler.txt", metn)
}
fn oxu(izah: &str) -> String{

    println!("{izah}");

    let mut giris = String::new();

    match io::stdin().read_line(&mut giris){
        Ok(_) => giris.trim().to_string(),
        Err(_) => String::new(),
    }
}
fn main() {

    let mut telebeler = fayildan_oxu();

    loop{
        println!("\n1 - Telebelerin siyahisi:");
        println!("2 - Telebe elave et");
        println!("3 - Fayla saxla");
        println!("4 - Cixis");

        let secim = oxu("Secim edin:");

        match secim.as_str(){
            "1" =>{
                for t in &telebeler{
                    println!("{}, {}, {}.", t.ad, t.yas, t.ixtisas);
                }
            }
            "2" => {
                let ad = oxu("Ad");
                let yas_metni = oxu("Yas");
                let ixtisas = oxu("Ixtisas");

                match yas_metni.parse::<i32>(){
                    Ok(yas) =>{
                        telebeler.push(Telebe { ad, yas, ixtisas });
                        println!("Elave edildi.");
                    }
                    Err(_) =>println!("Yas ededi duz deyil"),
                }
            }
            "3" =>{
                match fayla_yaz(&telebeler){
                    Ok(_) =>println!("Saxlanlidi"),
                    Err(e) =>println!("Xeta: {e}"),
                }
            }
            "4" =>{
                println!("Gorusenedek!");
                break;
            }
            _ =>println!("Bele secim yoxdur."),
        }
    }
}