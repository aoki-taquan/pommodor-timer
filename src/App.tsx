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

  const [junkisruning, setjunkisruning] = useState(true);

  const buttons = [
    {
      value: 5,
      display: "5 minutes",
    },
    {
      value: 25,
      display: "25 minutes",
    },
  ];

  const toggleTimer = () => {
    setTimerStart(!timerStart);
    emit_masseage(!timerStart ? "restart" : "pause");
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


  function emit_masseage(masseage: string) {
    emit("event-name", masseage)
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

    async function g() {
      unlisten = await listen('is_runing', (event) => {
        if (typeof event.payload === "boolean") {
          setTimerStart(event.payload);
          setjunkisruning(event.payload);
        }
      });
    }
    g();
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
          {/* <Button
            background="blue.300"
            marginX={5}
            onClick={triggerResetDialog}
          >
            Reset
          </Button> */}
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
                emit_masseage("start_" + value);
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