const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

//uma operação matematica simples de segundos em horas
fn variaveis(){

    let total =30;
    let total_em_segundos = total * SECONDS_IN_HOUR;

    println!("Total de horas trabalhadas {}", total);
    println!("Total de horas trabalhadas em segundos: {}",total_em_segundos);
}
