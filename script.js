var password = document.getElementById("password");
var generated = false;
var generate_count = 0;

function copyPassword() {
    var copyText = document.getElementById("password");
    if (copyText.value == "") {
        genPassword();
        copyPassword();
    }
    copyText.select();
    document.execCommand("copy");
}

// Shhhhh.... Easter Egg Code
// function easterEgg() {
//   var name = prompt("Hello there. What is your name?");
//   if (name == null || name == "") alert("I don't know how to greet someone who has no name but I've been watching you and I noticed that you generated 50 passwords in a row.");
//   else alert("Hello, " + name + ". I've been watching you and I noticed how you generated 50 passwords in a row.");
//   alert("My name is Rick. And I decide to give you a reward.");
//   if (window.confirm("Would you like to accept the reward?")) window.open("https://www.youtube.com/watch?v=dQw4w9WgXcQ", "_blank");
//   else alert("You recieved a new text message. Sender: Rick Astley.\n\nYou don't want my gift?");
// }

function genPassword() {
    var chars = "0123456789abcdefghijklmnopqrstuvwxyz!@#$%^&*()ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    while (Math.floor(Math.random() * 20) != Math.floor(Math.random() * 20)) {
        chars = shuffle(chars);
    }
    var passwordLength = 20;
    var password = "";
    for (var i = 0; i <= passwordLength; i++) {
        var randomNumber = Math.floor(Math.random() * chars.length);
        password += chars.substring(randomNumber, randomNumber + 1);
    }
    document.getElementById("password").value = password;
    // Easter Egg code.
    generate_count++;
    if (generate_count == 50) easterEgg();
}

function shuffle(s) {
    var arr = s.split('');
    arr.sort(function() {
        return 0.5 - Math.random();
    });
    s = arr.join('');
    return s;
}

function goBack() { history.go(-1); }

function getHelp() {
    if (window.confirm("This will open in new tab, continue?")) {
        window.open("https://sites.google.com/view/passgenhelppage/home", "_blank");
    }
}
