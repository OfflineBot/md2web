
pub fn css_template() -> String {
    "
@import url('https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap');

* {
    font-family: \"Roboto\", sans-serif;
}

html, body {
    background-color: #1d1d1d;
    color: #d8d8d8;
    padding-left: 2%;
    padding-right: 2%;
} 

li {
    padding: 2px;
    margin-bottom: 10px;
    font-size: 18px;
}
    
a {
    color: rgb(180, 104, 241);
}

h1 {
    font-size: 40px;
}
h2 {
    font-size: 30px;
}

p {
    font-size: 18px;
    margin: 5px;
} 
    ".to_string()
}