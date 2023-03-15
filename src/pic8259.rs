//! # What is the 8259 PIC?
//! The 8259 PIC (or Programmable Interrupt Controller) is a crucial component of the x86 architecture.
//! It led to the x86 architecture becoming interrupt-driven. Its purpose is to manage hardware interrupts
//! by sending them to the appropriate system interrupt. This allows the system to repond to the needs of devices
//! without losing time. In modern systems, the 8259 PIC has been replaced by the APIC (Advanced Programmable Interrupt Controller),
//! which is usable with multiple cores/processors.
//!
//! ## What exactly does the 8259 PIC do?
//! The 8259 PIC controls the interrupt mechanism of the CPU. It does this by feeding multiple interrupt requests, in order, to the
//! processor. A hardware interrupt will send a pulse along its interrupt line to the 8259 PIC. The 8259 PIC will then
//! translate the IRQ (Interrupt Request)/Hardware Interrupt into a system interrupt. It will then send a message to the CPU, interrupting
//! whatever task it was doing. The OS kernel should handle these IRQs and perform the necessary procedure (like polling the keyboard for a scancode)
//! or alert a userspace program of an interrupt by sending a message to a driver.
//!
//! ## What would be different if the 8259 PIC didn't exist?
//! Without the 8259 PIC, you would have to manually poll devices in the system to figure out if they want to do anything. You
//! would then waste time trying to go to these devices to figure out what they want to do. The 8259 PIC makes it easy by allowing the devices
//! to present themselves to you when they are ready to carry out an event.
//!
//! ## How does the 8259 PIC work?
//! Modern systems contain 2 8259 PICs, each with 8 inputs. One is called the "master" and the other is called the "slave". If any input on the PIC is raised,
//! it sets a bit interanlly that signals that the input needs servicing. Next, it checks if that channel is masked or not, and whether an interrupt is already pending.
//! If the channel is unmasked and no interrupt is pending, the PIC raises the interrupt line. The slave then feeds the IRQ number to the master and the master connects to the interrupt line.
//! After the processor accepts the interrupt, the master checks which of the PICs is reponsible for answering. It then eithr feeds the interrupt number to the processor or asks the slave to.
//! The PIC that answers, whether it be the master or slave, looks for the "vector offet" and adds it to the input line to compute the interrupt number. The processor
//! then acts on that interrupt address.
//!
//! ## Where can I read more?
//! The following links are useful to learning more about the 8259 PIC and interrupts:
//! - [8259 PIC on OSDev Wiki](https://wiki.osdev.org/PIC) (the page followed to write this module and where most of the documentation above is from)
//! - [8259 PIC on Wikipedia](https://en.wikipedia.org/wiki/Intel_8259)
//! - [Interrupts](https://wiki.osdev.org/IRQ)
//!
//! # The Public API
//! The API of this module is based off of the already existing [pic8259](https://github.com/rust-osdev/pic8259) crate.
