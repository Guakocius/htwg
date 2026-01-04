import librosa
from pytube import YouTube
from moviepy import AudioFileClip, VideoFileClip
import soundfile as sf
from IPython.display import Audio

url = "https://www.youtube.com/watch?v=1T14eOUf-28" # Song
start = 10
duration = 10
wav_out = "song.wav"
sr = 22050 # default sampling rate in librosa

yt = YouTube(url)
stream = yt.streams.filter(only_audio=True).first()
audio_path = stream.download(filename="./src/audio.mp4")

clip = AudioFileClip(audio_path)
subclip = clip.subclipped(start, start + duration)

temp = "temp.wav"
subclip.write_audiofile(temp, codec="pcm_s16le")
y, sr = librosa.load(temp, sr=sr, mono=True)
T = librosa.get_duration(path=temp)
print("Audio shape: ", y.shape)
print("Abtastfrequenz: ", sr, "Hz")
print("Dauer: ", T, 's')

Audio(data=y, rate=sr)

