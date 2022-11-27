import os, telebot, requests, json, csv
from dotenv import load_dotenv


load_dotenv()

key=os.getenv("mykey")
apitok=os.getenv("apitoken")
bot = telebot.TeleBot(apitok)

def csvExport(data):
    # filename = f"{data[0]}.csv"
    # with open(filename, 'w') as csvfile:
    #     csvwriter = csv.writer(csvfile) 
    #     csvwriter.wta[0]}.csv"
    filename = "movie.csv"

    with open(filename, 'w') as csvfile:
        csvwriter = csv.writer(csvfile) 
        csvwriter.writerow(data)

def getmovie(name):
    movieName=name.strip("/movie ")
    urlData=requests.get(f"http://www.omdbapi.com/?t={movieName}&apikey={key}")
    urlData=urlData.json()
    title,year,rating,photo=urlData["Title"],urlData["Year"],urlData["imdbRating"],urlData["Poster"]
    output=f'{photo}\nTitle:{title}\nYear:{year}\nimDbRating:{rating}'
    bot.send_message(message.chat.id,output)
    csvExport([title,year,rating])
    return output

@bot.message_handler(commands=['start', 'hello'])
def greet(message):
    global botRunning
    botRunning = True
    bot.reply_to(
        message, 'Hello there! I am a bot that will show movie information for you and export it in a CSV file.\n\n')
    
@bot.message_handler(commands=['stop', 'bye'])
def goodbye(message):
    global botRunning
    botRunning = False
    bot.reply_to(message, 'Bye!\nHave a good time')
    


@bot.message_handler(func=lambda message: botRunning, commands=['help'])
def helpProvider(message):
    bot.reply_to(message, '1.0 You can use \"/movie MOVIE_NAME\" command to get the details of a particular movie. For eg: \"/movie The Shawshank Redemption\"\n\n2.0. You can use \"/export\" command to export all the movie data in CSV format.\n\n3.0. You can use \"/stop\" or the command \"/bye\" to stop the bot.')


@bot.message_handler(func=lambda message: botRunning, commands=['movie'])
def getMovie(message):
    bot.reply_to(message, 'Getting movie info...')
    out=getmovie(message.text)
    bot.reply_to(message,out)

  
@bot.message_handler(func=lambda message: botRunning, commands=['export'])
def getList(message):
    bot.reply_to(message, 'Generating file...')
    file= open("movie.csv","rb")
    bot.send_document(message.chat.id,file)

@bot.message_handler(func=lambda message: botRunning)
def default(message):
    bot.reply_to(message, 'I did not understand '+'\N{confused face}')
    
bot.infinity_polling()
