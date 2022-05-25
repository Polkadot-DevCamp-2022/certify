/*eslint no-unused-vars: "off"*/

import React, { useState } from 'react'
import { Form, Input, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'
import { blake2AsHex } from '@polkadot/util-crypto'

function Main(props) {
  // The transaction submission status
  const [status, setStatus] = useState('')

  const [fileHash, setFileHash] = useState('')
  const [issuer, setIssuer] = useState('')

  const handleFileChange = event => {
    const fileReader = new FileReader()
    fileReader.readAsArrayBuffer(event.target.files[0])
    fileReader.onload = e => {
      setFileHash(blake2AsHex(new Uint8Array(e.target.result)))
    }
  }

  let result = ''
  if (status === '' || fileHash === '') {
    result = ''
  } else if (status !== 'None' && status === issuer) {
    result = 'Verified'
  } else {
    result = 'Not Verified'
  }

  return (
    <Grid.Column width={8}>
      <h1>Verify</h1>
      <Form>
        <Form.Field>
          <Input
            value={issuer}
            label="Issuer Account ID"
            placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
            type="text"
            onChange={(_, { value }) => setIssuer(value)}
          />
        </Form.Field>
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
          <TxButton
            label="Verify"
            type="QUERY"
            color="green"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'certify',
              callable: 'certificateMap',
              inputParams: [fileHash],
              paramFields: [true],
            }}
          ></TxButton>
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{result}</div>
      </Form>
    </Grid.Column>
  )
}

export default function Verify(props) {
  const { api } = useSubstrateState()
  return api.tx.certify ? <Main {...props} /> : null
}
