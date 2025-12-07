# -*- coding: utf-8 -*-
import pyaudio
import struct
import math
import numpy, matplotlib.pyplot as plt, pandas as pd, csv

INITIAL_TAP_THRESHOLD = 0.010
FORMAT = pyaudio.paInt16
SHORT_NORMALIZE = (1.0/32768.0)
CHANNELS = 2
RATE = 44100
INPUT_BLOCK_TIME = 0.05
INPUT_FRAMES_PER_BLOCK = int(RATE*INPUT_BLOCK_TIME)
# if we get this many noisy blocks in a row, increase the threshold
OVERSENSITIVE = 15.0/INPUT_BLOCK_TIME
# if we get this many quite blocks in a row, decrease the threshold
UNDERSENSITIVE = 120.0/INPUT_BLOCK_TIME
# if th enoise was longer than this many blocks, it’s not a ’tap’
MAX_TAP_BLOCKS = 0.15/INPUT_BLOCK_TIME
def get_rms( block ):
    # RMS amplitude is defined as the square root of the
    # mean over time of the square of the amplitude.
    # So we need to convert this string of bytes into
    # a string of 16-bit samples...
    # we will get one short out for each
    # two xhars in the string.
    count = len(block)/2
    format = "%dh"%(count)
    shorts = struct.unpack( format, block )
    
    # iterate over the block.
    sum_squares = 0.0
    for sample in shorts:
        # sample is a signed short in +/- 32728.
        # normalize it to 1.0
        n = sample * SHORT_NORMALIZE
        sum_squares += n * n
    return math.sqrt( sum_squares / count )
class TapTester(object):
    def __init__(self):
        self.pa = pyaudio.PyAudio()
        self.stream = self.open_mic_stream()
        self.tap_threshold = INITIAL_TAP_THRESHOLD
        self.noisycount = MAX_TAP_BLOCKS+1
        self.quietcount = 0
        self.errorcount = 0
    def stop(self):
        self.stream.close()
    def find_input_device(self):
        device_index = None
        for i in range( self.pa.get_device_count() ):
            devinfo = self.pa.get_device_info_by_index(i)
            print( "Device %d: %s"%(i,devinfo["name"]) )
        for keyword in ["mic","input"]:
            if keyword in devinfo["name"].lower():
                print( "Found an input: device %d - %s"%(
                    	i,devinfo["name"]) )
                device_index = i
                return device_index

        if device_index == None:
            print( "No prefeered input found; using deafult input device." )
        return device_index
    def open_mic_stream(self):
        device_index = self.find_input_device()
        stream = self.pa.open( format = FORMAT,
            channels = CHANNELS,
            rate = RATE,
            input = True,
            input_device_index =
            device_index,
            frames_per_buffer =
            INPUT_FRAMES_PER_BLOCK )
        return stream
    
    def tapDetected(self):
        print( "Tap!" )
    def listen(self):
        try:
            block = self.stream.read(INPUT_FRAMES_PER_BLOCK)
        except IOError as e:
            # dammit.
            self.errorcount += 1
            print( "(%d) Error recording: %s"%(self.
            errorcount,e) )
            self.noisycount += 1
            return

        amplitude = get_rms( block )
        if amplitude > self.tap_threshold:
            # noisy block
            self.quietcount = 0
            self.noisycount += 1
            if self.noisycount > OVERSENSITIVE:
                # tunr down the sensitivity
                self.tap_threshold *= 1.1
            else:
                # quiet block.
                if 1 <= self.noisycount <= MAX_TAP_BLOCKS:
                    self.tapDetected()
                self.noisycount = 0
                self.quietcount += 1
                if self.quietcount > UNDERSENSITIVE:
                    # turn up the sensitivity
                    self.tap_threshold *= 0.9
                    
if __name__ == "__main__":
    tt = TapTester()
    FORMAT = pyaudio.paInt16
    SAMPLEFREQ = 44100
    FRAMESIZE = 1024
    NOFFRAMES = 220
    p = pyaudio.PyAudio()
    print('running')
    stream = p.open(format=FORMAT,channels=1,rate=SAMPLEFREQ,
    input=True,frames_per_buffer=FRAMESIZE)
    data = stream.read(NOFFRAMES*FRAMESIZE)
    decoded = numpy.fromstring(data, int);
    stream.stop_stream()
    stream.close()
    p.terminate()
    print('done')
    plt.plot(decoded)
    plt.show()
    
    #filename = "./plot_harmonica_links5.csv"
    
    filename = [f"./plot_harmonica_hoch{i}", f"./plot_harmonica_tief{i}", f"./plot_harmonica_links{i}", f"./plot_harmonica_rechts{i}" for i in range(1, 6)]
    print("Filename\n",filename)
    with open(filename, "w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["Time", "Frequency"])
        writer.writerows(zip([i for i in range(0, 50001)], decoded))
    
    #for i in range(1000):
    #    tt.listen()
    df = pd.read_csv(filename)
    df = df.drop([i for i in df["Time"] if i < 8000])
    print(df)
    df.to_csv("./plot_harmonica_links5_normalized.csv", index=False)
        