#include "roughguardwindow.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QLineEdit>
#include <QFileDialog>
#include <QPixmap>
#include <QScrollArea>
#include <QMessageBox>
#include <QDir>
#include <QProcess>
#include <QStandardPaths>
#include <QFile>
#include <QTextStream>

RoughGuardWindow::RoughGuardWindow(QWidget* parent)
    : QWidget(parent)
{
    setWindowTitle("Roguh Guard");
    
    // Set minimum size for the window
    setMinimumSize(800, 80);
    
    // Create the main layout
    m_mainLayout = new QVBoxLayout(this);
    
    // Create buttons layout
    QHBoxLayout* buttonLayout = new QHBoxLayout();
    
    // Create and add the "Add File" button
    m_addPGNButton = new QPushButton("Load PGN", this);
    m_addPGNButton->setMaximumWidth(120);
    buttonLayout->addWidget(m_addPGNButton);
    
    // Create and add the "Send" button
    m_sendButton = new QPushButton("Send", this);
    m_sendButton->setMaximumWidth(120);
    m_sendButton->setEnabled(false); // Initially disabled
    buttonLayout->addWidget(m_sendButton);
    
    // Add stretch to push buttons to the left
    buttonLayout->addStretch();
    
    m_mainLayout->addLayout(buttonLayout);
    
    // Add stretch to push buttons to the left
    buttonLayout->addStretch();
    
    m_mainLayout->addLayout(buttonLayout);
    
    // Create and add the path text field
    QLabel* pathLabel = new QLabel("Photo Path:", this);
    m_pathLineEdit = new QLineEdit(this);
    m_pathLineEdit->setReadOnly(true);
    m_pathLineEdit->setPlaceholderText("No photo selected...");
    
    m_mainLayout->addWidget(pathLabel);
    m_mainLayout->addWidget(m_pathLineEdit);
    
    // Create and add the result text field
    QLabel* resultLabel = new QLabel("Result from Rust:", this);
    m_resultLineEdit = new QLineEdit(this);
    m_resultLineEdit->setReadOnly(true);
    m_resultLineEdit->setPlaceholderText("No result yet...");
    
    m_mainLayout->addWidget(resultLabel);
    m_mainLayout->addWidget(m_resultLineEdit);
    
    
    
    // Connect the button signals to the slots
    connect(m_addPGNButton, &QPushButton::clicked, this, &RoughGuardWindow::onAddFileClicked);
    connect(m_sendButton, &QPushButton::clicked, this, &RoughGuardWindow::onSendClicked);

    // Set the layout to the widget
    setLayout(m_mainLayout);
    
    // Make it a proper window (not embedded)
    setWindowFlags(Qt::Window);
}

void RoughGuardWindow::onAddFileClicked()
{
    // Open file dialog to select any file
    QString fileName = QFileDialog::getOpenFileName(
        this,
        "Select File",
        QDir::homePath(),
        "All Files (*.*)"
    );

    if (!fileName.isEmpty())
    {
        // Display the file path in the text field
        m_pathLineEdit->setText(fileName);

        // Optional: Update label text to show selected file name
        QFileInfo fileInfo(fileName);
        // Enable send button or other actions as needed
        m_sendButton->setEnabled(true);

    }
    else{
        // Show error message if no file was selected
        QMessageBox::warning(this, "Error", "No file selected.");
        m_pathLineEdit->clear();
        m_sendButton->setEnabled(false);
    }
}

void RoughGuardWindow::onSendClicked()
{
    QString imagePath = m_pathLineEdit->text();
    
    if (imagePath.isEmpty())
    {
        QMessageBox::warning(this, "Error", "No file path to send.");
        return;
    }
    
    // Path to the existing Rust project
    QString rustProjectPath = "/home/sasa/My_Projects/Graduation_Project/Test_Work/test2";
    
    // Clear the result field
    m_resultLineEdit->clear();
    m_resultLineEdit->setPlaceholderText("Processing...");
    
    // Try to run the Rust project with the image path as an argument
    QProcess* process = new QProcess(this);
    process->setWorkingDirectory(rustProjectPath);
    
    // Connect to handle the process completion
    connect(process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
        [this, process, imagePath](int exitCode, QProcess::ExitStatus exitStatus) {
            if (exitStatus == QProcess::NormalExit && exitCode == 0)
            {
                // Read the output from the Rust program
                QString output = process->readAllStandardOutput().trimmed();
                
                // Display the result in the result text field
                if (!output.isEmpty())
                {
                    m_resultLineEdit->setText(output);
                    QMessageBox::information(this, "Success", 
                        QString("Received result from Rust project:\n%1").arg(output));
                }
                else
                {
                    m_resultLineEdit->setText("No output received");
                    QMessageBox::information(this, "Success", "Rust project executed successfully but no output received.");
                }
            }
            else
            {
                QString error = process->readAllStandardError();
                m_resultLineEdit->setText("Error occurred");
                m_resultLineEdit->setPlaceholderText("Error occurred");
                QMessageBox::warning(this, "Error", 
                    QString("Failed to run Rust project.\nError: %1").arg(error));
            }
            process->deleteLater();
        });
    
    // Start the Rust process
    QStringList arguments;
    arguments << "run" << "--" << imagePath;
    process->start("cargo", arguments);
    
    if (!process->waitForStarted(3000))
    {
        QMessageBox::critical(this, "Error", "Could not start the Rust project. Make sure Rust and Cargo are installed.");
        process->deleteLater();
    }
}
