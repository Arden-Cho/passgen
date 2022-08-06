# Passgen main.py script
import time
import string
import secrets


def prompt(prompt_type, text):
    if prompt_type == "e":
        print("[Error]", text)
        return
    elif prompt_type == "w":
        print("[Warning]", text)
        return
    elif prompt_type == "i":
        print("[Information]", text)
        return
    elif prompt_type == "c":
        prompt_choose_result = input("[Choose] " + text)
        return prompt_choose_result
    return


def abort(sleep_time):
    prompt("i", "This application will be aborted in " + str(sleep_time) + " seconds because of a critical error.")
    time.sleep(sleep_time)
    quit()


def get_random_password(length):
    random_password = ''.join(secrets.choice(string.ascii_letters + string.digits + string.punctuation) for x in range(length))
    return random_password


print("Hello and welcome to passgen, a open-source and lightning-fast password generator written in Python.")
while True:
    password_length = input("Please enter the length of password you want to generate (Default and Recommended: 12)\n"
                            "Leave blank for default: ")
    if password_length == "" or password_length == "12":
        prompt("i", "Password length set to default (12).")
        password_length = 12
        time.sleep(0.3)
        break
    elif password_length.isdigit():
        if int(password_length) < 150:
            prompt("i", "Password length set to " + password_length + ".")
            time.sleep(0.3)
            if int(password_length) < 9:
                prompt("w", "Passwords shorter than 8 digits is considered to be not safe and can be cracked by "
                            "hackers, we recommend you to use a longer password for better security.")
                time.sleep(0.3)
        break
    else:
        prompt("e", "Please double-check your input, the input should be a number and not containing decimals.")
        time.sleep(0.3)
        while True:
            input_error_prompt_user_input = prompt("c", "Please choose a option [a: abort, r: re-enter]: ")
            if input_error_prompt_user_input.lower() == "a":
                abort(3)
            elif input_error_prompt_user_input.lower() == "r":
                break
            else:
                prompt("w", "\"" + input_error_prompt_user_input + "\" is not a valid option. Please retry.")
                continue
prompt("i", "Creating password...")
time.sleep(0.3)
password = get_random_password(int(password_length))
print("Password created. Your random password is: \n\n" + password)
print("\nYou can copy the password by right-clicking or Command/Control + C")
input("Press ENTER to finish.")


