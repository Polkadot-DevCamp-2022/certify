/*eslint no-unused-vars: "off"*/

import React, { useState, useEffect, useRef } from 'react'
import { Form, Input, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'
import { ContractPromise } from '@polkadot/api-contract'
import { Button } from 'semantic-ui-react'
import { blake2AsHex } from '@polkadot/util-crypto'
import config from './config'

function Main(props) {
  const { api, currentAccount } = useSubstrateState()

  // The transaction submission status
  const [status, setStatus] = useState('')

  const [fileHash, setFileHash] = useState('')
  const contractRef = useRef(null)

  useEffect(() => {
    if (!api) return

    const contract = new ContractPromise(api, config.abi, config.CONTRACT_ADDR)
    contractRef.current = contract
  }, [api])

  const handleFileChange = event => {
    const fileReader = new FileReader()
    fileReader.readAsArrayBuffer(event.target.files[0])
    fileReader.onload = e => {
      setFileHash(blake2AsHex(new Uint8Array(e.target.result)))
    }
  }

  const handleIssue = () => {
    setStatus('Current transaction status: Ready')

    contractRef.current.tx
      .issue({ value: 0, gasLimit: -1 }, fileHash)
      .signAndSend(currentAccount, result => {
        if (result.status.isInBlock) {
          setStatus('Current transaction status: In a Block')
        } else if (result.status.isFinalized) {
          setStatus('Current transaction status: Finalized')
        }
      })
  }

  const handleRevoke = () => {
    setStatus('Current transaction status: Ready')

    contractRef.current.tx
      .revoke({ value: 0, gasLimit: -1 }, fileHash)
      .signAndSend(currentAccount, result => {
        if (result.status.isInBlock) {
          setStatus('Current transaction status: In a Block')
        } else if (result.status.isFinalized) {
          setStatus('Current transaction status: Finalized')
        }
      })
  }

  return (
    <Grid.Column width={8}>
      <h1>Contract Module</h1>
      <p>{config.CONTRACT_ADDR}</p>
      <Form>
        <Form.Field>
          <Input
            label="Certificate File"
            type="file"
            onChange={handleFileChange}
          />
        </Form.Field>
        <Form.Field>
          <Input
            value={fileHash}
            label="Certificate Hash"
            placeholder="0x0000... 256 bits"
            state="hashValue"
            type="text"
            onChange={(_, { value }) => setFileHash(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <Button basic color={'red'} type="submit" onClick={handleRevoke}>
            Revoke
          </Button>
          <Button basic color={'blue'} type="submit" onClick={handleIssue}>
            Issue
          </Button>
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  )
}

export default function Contract(props) {
  const { api } = useSubstrateState()
  return api.tx.certify ? <Main {...props} /> : null
}
