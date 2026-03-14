export interface Contact {
  email?: string
  phone?: string
  addr1?: string
  addr2?: string
  city?: string
  state?: string
  zip?: string
}

export interface Company {
  id: { 0: number }
  name: string
  logo?: string
  contact: Contact
}

export interface Client {
  id: { 0: number }
  name: string
  contact: Contact
}

export interface Terms {
  id: { 0: number }
  name: string
  days: number
}

export interface Method {
  id: { 0: number }
  name: string
  link?: string
  qr?: string
}

export interface Template {
  id: { 0: number }
  name: string
  company: Company
  client: Client
  terms: Terms
  method: Method[]
}

export interface Item {
  id: { 0: number }
  name: string
  rate: { 0: string }
}

export type PaidStatis =
  | 'Waiting'
  | 'PastDue'
  | { Paid: { date: string; check: string | null } }
  | { Failed: { date: string } }
  | { Refunded: { date: string } }

export interface Invoice {
  id: { 0: number }
  template: Template
  attributes: {
    show_methods: boolean
    show_notes: boolean
    stage: 'Quote' | 'Invoice'
    status: PaidStatus
  }
  date: string
  notes?: string
  items: Record<string, string>
