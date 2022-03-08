let world: string = "Olá mundo"

class Question {

    public multiplier(x: number, y: number): number {
        let result: number = (x * y);
        return result;
    }
    
    public print_string_to_upper(str: string): void {
        console.log(str.toUpperCase());
    }
}

let main: void = (function (){
    // Comentário
    console.log(world);
    let question = new Question();
    console.log(question.multiplier(2, 2));
    question.print_string_to_upper("thiago");

}())
