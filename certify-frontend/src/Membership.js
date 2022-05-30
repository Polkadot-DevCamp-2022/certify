/*eslint no-unused-vars: "off"*/

import React, { useState, useEffect, useRef } from 'react'
import { Form, Input, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { ContractPromise } from '@polkadot/api-contract'
import { Button } from 'semantic-ui-react'
import config from './config'

function Main(props) {
  const { api, currentAccount } = useSubstrateState()

  // The transaction submission status
  const [status, setStatus] = useState('')

  const [accountId, setAccountId] = useState('')
  const contractRef = useRef(null)
  const [isOwner, setOwnership] = useState(false)

  useEffect(() => {
    if (!api) return

    const contract = new ContractPromise(api, config.abi, config.CONTRACT_ADDR)
    contractRef.current = contract
  }, [api])

  useEffect(() => {
    console.log(currentAccount)
    if (!contractRef.current || !currentAccount) return
    ;(async () => {
      const { result, output } = await contractRef.current.query.getOwner(
        currentAccount.address,
        { value: 0, gasLimit: -1 }
      )

      // check if the call was successful
      if (result.isOk) {
        setOwnership(output.toHuman() === currentAccount.address)
      } else {
        setOwnership(false)
      }
    })()
  }, [currentAccount])

  const handleAdd = () => {
    setStatus('Current transaction status: Ready')

    contractRef.current.tx
      .addMember({ value: 0, gasLimit: -1 }, accountId)
      .signAndSend(currentAccount, result => {
        if (result.status.isInBlock) {
          setStatus('Current transaction status: In a Block')
        } else if (result.status.isFinalized) {
          setStatus('Current transaction status: Finalized')
        }
      })
  }

  const handleRemove = () => {
    setStatus('Current transaction status: Ready')

    contractRef.current.tx
      .removeMember({ value: 0, gasLimit: -1 }, accountId)
      .signAndSend(currentAccount, result => {
        if (result.status.isInBlock) {
          setStatus('Current transaction status: In a Block')
        } else if (result.status.isFinalized) {
          setStatus('Current transaction status: Finalized')
        }
      })
  }

  const handleCheck = () => {
    if (!currentAccount || !accountId) return
    ;(async () => {
      const { result, output } = await contractRef.current.query.isMember(
        currentAccount.address,
        { value: 0, gasLimit: -1 },
        accountId
      )

      // check if the call was successful
      if (result.isOk) {
        setStatus(`Membership: ${output.toHuman() ? 'Yes' : 'No'}`)
      } else {
        setStatus('Error', result.asErr)
      }
    })()
  }

  return (
    <Grid.Column width={8}>
      <h1>Organization Membership</h1>
      <p>{config.CONTRACT_ADDR}</p>
      <p>Ownership Status: {isOwner ? 'True' : 'False'}</p>
      <Form>
        <Form.Field>
          <Input
            value={accountId}
            label="Account Id"
            placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
            type="text"
            onChange={(_, { value }) => setAccountId(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <Button
            basic
            color={'red'}
            type="submit"
            onClick={handleRemove}
            disabled={!isOwner}
          >
            Remove Member
          </Button>
          <Button
            basic
            color={'blue'}
            type="submit"
            onClick={handleAdd}
            disabled={!isOwner}
          >
            Add Member
          </Button>
          <Button basic color={'green'} type="submit" onClick={handleCheck}>
            Is Member
          </Button>
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  )
}

export default function Membership(props) {
  const { api } = useSubstrateState()
  return api.tx.certify ? <Main {...props} /> : null
}
