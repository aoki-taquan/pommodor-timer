import { Button, Flex, Text } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { sendNotification } from "@tauri-apps/api/notification";
import { ask } from "@tauri-apps/api/dialog";
import { listen } from '@tauri-apps/api/event'
import { event } from "@tauri-apps/api";
import { emit } from '@tauri-apps/api/event'



// const unlisten = await listen('k-to-front', (event) => {
//   console.log("hogehoge");
// })

function App() {
  const [time, setTime] = useState(0);

  const [timerStart, setTimerStart] = useState(false);

  const [junk, setJunk] = useState(0);

  const buttons = [
    {
      value: 900,
      display: "15 minutes",
    },
    {
      value: 1800,
      display: "30 minutes",
    },
    {
      value: 3600,
      display: "60 minutes",
    },
  ];

  const toggleTimer = () => {
    setTimerStart(!timerStart);
  };

  const triggerResetDialog = async () => {
    let shouldReset = await ask("Do you want to reset timer?", {
      title: "Pomodoro Timer App",
      type: "warning",
    });
    if (shouldReset) {
      setTime(900);
      setTimerStart(false);
    }
  };


  function emit_masseage(masseage: string, time: number) {
    emit(masseage, time)
  }

  useEffect(() => {
    let unlisten: any;
    async function f() {
      unlisten = await listen('now-remining-time', (event) => {
        setTime(100);
        if (typeof event.payload === "number") {
          setJunk(event.payload);

        }
      });
    }
    f();
    return () => {
      f();
    }
  }, [])

  return (
    <div className="App" style={{ height: "100%" }}>
      <Flex
        background="gray.700"
        height="100%"
        alignItems="center"
        flexDirection="column"
      >
        <Text color="white" fontWeight="bold" marginTop="20" fontSize="35">
          Pomodoro Timer
        </Text>
        <Text fontWeight="bold" fontSize="7xl" color="white">
          {`${Math.floor(time / 60) < 10
            ? `0${Math.floor(time / 60)}`
            : `${Math.floor(time / 60)}`
            }:${time % 60 < 10 ? `0${time % 60}` : time % 60}`}
        </Text><Text fontWeight="bold" fontSize="7xl" color="white">
          {`${Math.floor(junk / 60) < 10
            ? `0${Math.floor(junk / 60)}`
            : `${Math.floor(junk / 60)}`
            }:${junk % 60 < 10 ? `0${junk % 60}` : junk % 60}`}
        </Text>
        <Flex>
          <Button
            width="7rem"
            background="tomato"
            color="white"
            onClick={toggleTimer}
          >
            {!timerStart ? "Start" : "Pause"}
          </Button>
          <Button
            background="blue.300"
            marginX={5}
            onClick={triggerResetDialog}
          >
            Reset
          </Button>
        </Flex>
        <Flex marginTop={10}>
          {buttons.map(({ value, display }) => (
            <Button
              marginX={4}
              background="green.300"
              color="white"
              onClick={() => {
                setTimerStart(false);
                setTime(value);
              }}
            >
              {display}
            </Button>
          ))}
        </Flex>
      </Flex>
    </div>
  );
}

export default App;