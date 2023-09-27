use mymac::print_each_token;



fn main() {
    print_each_token!(
        PARAMS 
            $firstname:String,
            $lastname:String
        SELECT
            FirstName as first_name:String,
            LastName as last_name:String,
            Email as email:String,
            Pets as pets:Vec<String>
        FROM
            Users
            INNER JOIN Pets ON Pets.UserId = Users.Id
        WHERE True
            AND Email != "jgarber@appcove.com"
            AND FirstName = $first_name
            #>>
        GROUP BY 
            first_name,
            last_name,
            email,
            pets
            +, -, *, /, %, ^, |/ , ||/, !, !!, @, =, <>, !=, <, >, <=, >=, !<, !>, &, |, #, ~, <<, >>, AND, OR, NOT, @@, @>, <@, ||, &&, ->, ->>, #>, #>>, @>, <@, &&, -|-


    );

    mymac::create_inline_struct_foo!{};

    let x = Foo {
        a: 10
    };

    println!("x is {:?}", x);
    
}
