/*eslint no-unused-vars: "off"*/

import React, { useState } from 'react'
import { Form, Input, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

function Main(props) {
  const { api } = useSubstrateState()

  // The transaction submission status
  const [status, setStatus] = useState('')

  const [formValue, setFormValue] = useState(0)

  return (
    <Grid.Column width={8}>
      <h1>Certify Module</h1>
      <Form>
        <Form.Field>
          <Input
            label="Certificate Hash"
            placeholder="0x0000... 256 bits"
            state="hashValue"
            type="text"
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            label="Revoke"
            type="SIGNED-TX"
            color="red"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'certify',
              callable: 'revoke',
              inputParams: [formValue],
              paramFields: [true],
            }}
          />
          <TxButton
            label="Issue"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'certify',
              callable: 'issue',
              inputParams: [formValue],
              paramFields: [true],
            }}
          />
          <TxButton
            label="Get Issuer"
            type="QUERY"
            color="green"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'certify',
              callable: 'certificateMap',
              inputParams: [formValue],
              paramFields: [true],
            }}
          ></TxButton>
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  )
}

export default function Certify(props) {
  const { api } = useSubstrateState()
  return api.tx.certify ? <Main {...props} /> : null
}
