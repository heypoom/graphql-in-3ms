import {check, sleep} from "k6"
import http from "k6/http"

const query = `
query Crustacean {
  crabs {
    level
    amount
  }

  lobsters {
    level
    amount
  }
}`

export default function loadTest() {
  const url = "http://localhost:8080/graphql"
  const body = JSON.stringify({query})
  const headers = {"Content-Type": "application/json"}

  const res = http.post(url, body, {headers})
  console.log("Response Time =", res.timings.duration, "ms")

  check(res, {"is status 200": (r) => r.status === 200})
  sleep(0.3)
}
