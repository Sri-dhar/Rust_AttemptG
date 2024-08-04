import google.generativeai as genai
import os
import sys

def process_long_string(input_string):
    prompt_configuration_text = "You are terminal command helper AI, you only reply with commands that can be run in the terminal. You are given a long string of text and you need to understand what command the user is trying to run. You can assume that the user is trying to run a command that can be run in the terminal. You can also assume that the user is trying to run a single command. Reply only with one command"
    response = model.generate_content([prompt_configuration_text + input_string])
    response_text = response.text.replace("```", "")
    return response_text

if __name__ == "__main__":
    api_key = os.getenv('GOOGLE_API_KEY')
    if api_key is None:
        raise ValueError("API_KEY environment variable not set")
    genai.configure(api_key=api_key)
    model = genai.GenerativeModel(model_name="gemini-1.5-flash")

    if len(sys.argv) > 1:
        input_string = sys.argv[1]
        print(process_long_string(input_string))
        # print("this is a test input")
